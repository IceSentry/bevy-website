use anyhow::bail;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

const BASE_URL: &str = "https://gitlab.com/api/v4/projects";

#[derive(Deserialize)]
pub struct GitlabProjectSearchResult {
    pub id: usize,
    pub default_branch: String,
}

pub struct GitlabClient {
    client: reqwest::blocking::Client,
    // This is not currently used because we have so few assets using gitlab that we don't need it.
    _token: String,
}

impl GitlabClient {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            _token: token,
        }
    }

    /// Finds a list of repo based on their name
    /// Useful to get the repo id and default_branch
    pub fn search_project_by_name(
        &self,
        repository_name: &str,
    ) -> anyhow::Result<Vec<GitlabProjectSearchResult>> {
        let response = self
            .client
            .get(BASE_URL)
            .query(&[("search", repository_name)])
            .header(ACCEPT, "application/json")
            .header(USER_AGENT, "bevy-website-generate-assets")
            // .bearer_auth(self.token.clone())
            .send()?;

        Ok(response.json()?)
    }

    /// Gets the content of a file from a gitlab repo
    pub fn get_content(
        &self,
        id: usize,
        default_branch: &str,
        content_path: &str,
    ) -> anyhow::Result<String> {
        let response = self
            .client
            .get(format!("{BASE_URL}/{id}/repository/files/{content_path}"))
            .query(&[("ref", default_branch)])
            .header(ACCEPT, "application/json")
            .header(USER_AGENT, "bevy-website-generate-assets")
            // .bearer_auth(self.token.clone())
            .send()?;

        #[derive(Deserialize)]
        struct GitlabContentResponse {
            encoding: String,
            content: String,
        }

        let json: GitlabContentResponse = response.json()?;

        if json.encoding == "base64" {
            let data = base64::decode(json.content.replace('\n', "").trim())?;
            Ok(String::from_utf8(data)?)
        } else {
            bail!("Content is not in base64");
        }
    }
}
