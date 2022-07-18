use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

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

fn visit_dirs(dir: PathBuf, section: &mut Section) -> anyhow::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let client = reqwest::blocking::Client::new();
    let github_token = std::env::var("GITHUB_TOKEN");

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == ".github" {
            continue;
        }
        if path.is_dir() {
            let folder = path.file_name().unwrap();
            let (order, sort_order_reversed) = if path.join("_category.toml").exists() {
                let from_file: toml::Value =
                    toml::de::from_str(&fs::read_to_string(path.join("_category.toml")).unwrap())
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
            visit_dirs(path.clone(), &mut new_section)?;
            section.content.push(AssetNode::Section(new_section));
        } else {
            if path.file_name().unwrap() == "_category.toml"
                || path.extension().expect("file must have an extension") != "toml"
            {
                continue;
            }
            let mut asset: Asset = toml::de::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            asset.original_path = Some(path);

            println!("Getting extra metadata for {}", asset.name);

            let url = url::Url::parse(&asset.link)?;
            match url.host_str() {
                Some("github.com") => {
                    if let Ok(github_token) = &github_token {
                        let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
                        let username = segments[0];
                        let repository_name = segments[1];
                        get_metadata_from_github(
                            username,
                            repository_name,
                            &mut asset,
                            &client,
                            github_token,
                        )?;
                    }
                }
                Some("crates.io") => {
                    // TODO get crates.io metadata from <https://github.com/alyti/cratesio-dbdump-lookup>
                }
                _ => {}
            }

            section.content.push(AssetNode::Asset(asset));
        }
    }
    Ok(())
}

pub fn parse_assets(asset_dir: &str) -> anyhow::Result<Section> {
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
    )?;
    Ok(asset_root_section)
}

fn get_metadata_from_github(
    username: &str,
    repository_name: &str,
    asset: &mut Asset,
    client: &reqwest::blocking::Client,
    github_token: &str,
) -> anyhow::Result<()> {
    let response = client
        .get(format!(
            "https://api.github.com/repos/{username}/{repository_name}/contents/Cargo.toml"
        ))
        .header(ACCEPT, "application/json")
        .header(USER_AGENT, "bevy-website-generate-assets")
        .bearer_auth(github_token)
        .send();

    let response = match response {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{err}");
            return Ok(());
        }
    };

    #[derive(Deserialize)]
    struct GithubContentResponse {
        encoding: String,
        content: String,
    }

    let json: GithubContentResponse = match response.json() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{err}");
            return Ok(());
        }
    };

    // The github rest api is supposed to return the content as a base64 encoded string
    let content = if json.encoding == "base64" {
        String::from_utf8(base64::decode(json.content.replace('\n', "").trim())?)?
    } else {
        eprintln!("content is not in base64");
        return Ok(());
    };

    let cargo_manifest = toml::from_str::<cargo_toml::Manifest>(&content)?;

    // Get the license from the package information
    if let Some(cargo_toml::Package {
        license: Some(license),
        ..
    }) = &cargo_manifest.package
    {
        let licenses = license.split("OR").map(|x| x.trim().to_string()).collect();
        asset.licenses = Some(licenses);
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
