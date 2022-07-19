use anyhow::Context;
use cratesio_dbdump_csvtab::CratesIODumpLoader;
use github_client::GithubClient;
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

pub mod github_client;

type CratesIoDb = cratesio_dbdump_csvtab::rusqlite::Connection;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub name: String,
    pub link: String,
    pub description: String,
    pub order: Option<usize>,
    pub image: Option<String>,
    pub licenses: Option<Vec<String>>,
    pub bevy_versions: Option<Vec<String>>,

    // this field is not read from the toml file
    #[serde(skip)]
    pub original_path: Option<PathBuf>,
}

impl Asset {
    /// Parses a license string separated with OR into a Vec<String>
    fn set_license(&mut self, license: &str) {
        let licenses = license.split("OR").map(|x| x.trim().to_string()).collect();
        self.licenses = Some(licenses);
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub content: Vec<AssetNode>,
    pub template: Option<String>,
    pub header: Option<String>,
    pub order: Option<usize>,
    pub sort_order_reversed: bool,
}

#[derive(Debug, Clone)]
pub enum AssetNode {
    Section(Section),
    Asset(Asset),
}
impl AssetNode {
    pub fn name(&self) -> String {
        match self {
            AssetNode::Section(content) => content.name.clone(),
            AssetNode::Asset(content) => content.name.clone(),
        }
    }
    pub fn order(&self) -> usize {
        match self {
            AssetNode::Section(content) => content.order.unwrap_or(99999),
            AssetNode::Asset(content) => content.order.unwrap_or(99999),
        }
    }
}

fn visit_dirs(
    dir: PathBuf,
    section: &mut Section,
    crates_io_db: Option<&CratesIoDb>,
    github_client: Option<&GithubClient>,
) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == ".github" {
                continue;
            }
            if path.is_dir() {
                let folder = path.file_name().unwrap();
                let (order, sort_order_reversed) = if path.join("_category.toml").exists() {
                    let from_file: toml::Value = toml::de::from_str(
                        &fs::read_to_string(path.join("_category.toml")).unwrap(),
                    )
                    .unwrap();
                    (
                        from_file
                            .get("order")
                            .and_then(|v| v.as_integer())
                            .map(|v| v as usize),
                        from_file
                            .get("sort_order_reversed")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    )
                } else {
                    (None, false)
                };
                let mut new_section = Section {
                    name: folder.to_str().unwrap().to_string(),
                    content: vec![],
                    template: None,
                    header: None,
                    order,
                    sort_order_reversed,
                };
                visit_dirs(path.clone(), &mut new_section, crates_io_db, github_client)?;
                section.content.push(AssetNode::Section(new_section));
            } else {
                if path.file_name().unwrap() == "_category.toml"
                    || path.extension().expect("file must have an extension") != "toml"
                {
                    continue;
                }

                let mut asset: Asset = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                asset.original_path = Some(path);

                get_extra_metadata(&mut asset, crates_io_db, github_client)?;

                section.content.push(AssetNode::Asset(asset));
            }
        }
    }
    Ok(())
}

pub fn parse_assets(
    asset_dir: &str,
    crates_io_db: Option<&CratesIoDb>,
    github_client: Option<&GithubClient>,
) -> anyhow::Result<Section> {
    let mut asset_root_section = Section {
        name: "Assets".to_string(),
        content: vec![],
        template: Some("assets.html".to_string()),
        header: Some("Assets".to_string()),
        order: None,
        sort_order_reversed: false,
    };
    visit_dirs(
        PathBuf::from_str(asset_dir).unwrap(),
        &mut asset_root_section,
        crates_io_db,
        github_client,
    )?;
    Ok(asset_root_section)
}

/// Tries to get bevy supported version and license information from github or a crates.io database dump
fn get_extra_metadata(
    asset: &mut Asset,
    crates_io_db: Option<&CratesIoDb>,
    github_client: Option<&GithubClient>,
) -> anyhow::Result<()> {
    println!("Getting extra metadata for {}", asset.name);

    let url = url::Url::parse(&asset.link)?;
    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    match url.host_str() {
        Some("github.com") => {
            if let Some(client) = github_client {
                let username = segments[0];
                let repository_name = segments[1];

                if let Err(err) = get_metadata_from_github(asset, client, username, repository_name)
                {
                    eprintln!("Failed to get metadata from github for {}", asset.name);
                    eprintln!("ERROR: {err}")
                }
            }
        }
        Some("crates.io") => {
            if let Some(db) = crates_io_db {
                let crate_name = segments[1];

                if let Err(err) = get_metadata_from_crates_io_db(asset, db, crate_name) {
                    eprintln!("Failed to get metadata from github for {}", asset.name);
                    eprintln!("ERROR: {err}")
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn get_metadata_from_github(
    asset: &mut Asset,
    client: &GithubClient,
    username: &str,
    repository_name: &str,
) -> anyhow::Result<()> {
    let content = client
        .get_content(username, repository_name, "Cargo.toml")
        .context("Failed to get content from github")?;

    let cargo_manifest = toml::from_str::<cargo_toml::Manifest>(&content)?;

    // Get the license from the package information
    let license = if let Some(cargo_toml::Package { license, .. }) = &cargo_manifest.package {
        license.clone()
    } else {
        // If there's no license in the Cargo.toml, try to get it directly from the repo
        client.get_license(username, repository_name).ok()
    };

    if let Some(license) = license {
        asset.set_license(&license);
    }

    // Find any dep that starts with bevy and get the version
    // This makes sure to handle all the bevy_* crates
    let version = cargo_manifest
        .dependencies
        .keys()
        .find(|k| k.starts_with("bevy"))
        .and_then(|key| {
            cargo_manifest
                .dependencies
                .get(key)
                .and_then(get_bevy_version)
        });

    if let Some(version) = version {
        asset.bevy_versions = Some(vec![version]);
    }

    Ok(())
}

/// Gets the bevy version from the dependency list
/// Returns the version number if available.
/// If is is a git dependency, return either "main" or "git" for anything that isn't "main".
fn get_bevy_version(dep: &cargo_toml::Dependency) -> Option<String> {
    match dep {
        cargo_toml::Dependency::Simple(version) => Some(version.to_string()),
        cargo_toml::Dependency::Detailed(detail) => {
            if let Some(version) = &detail.version {
                Some(version.to_string())
            } else if detail.git.is_some() {
                if detail.branch == Some(String::from("main")) {
                    Some(String::from("main"))
                } else {
                    Some(String::from("git"))
                }
            } else {
                None
            }
        }
    }
}

/// Downloads the crates.io database dump and open a connection to the db
pub fn prepare_crates_db() -> anyhow::Result<CratesIoDb> {
    Ok(CratesIODumpLoader::default()
        .tables(&["crates", "dependencies", "versions"])
        .preload(true)
        .update()?
        .open_db()?)
}

/// Gets the required metadata from the crates.io database dump
fn get_metadata_from_crates_io_db(
    asset: &mut Asset,
    db: &CratesIoDb,
    crate_name: &str,
) -> anyhow::Result<()> {
    let rev_dependency = (cratesio_dbdump_lookup::get_rev_dependency(db, crate_name, "bevy")?)
        .into_iter()
        .flatten();
    for (_, _, license, _, deps) in rev_dependency {
        asset.set_license(&license);

        if let Ok(deps) = deps {
            if let Some((version, _)) = deps.first() {
                let version = version.clone().replace('^', "");
                asset.bevy_versions = Some(vec![version]);
            }
        }
    }
    Ok(())
}
