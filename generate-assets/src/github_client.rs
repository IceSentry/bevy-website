use anyhow::bail;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

const BASE_URL: &str = "https://api.github.com";

pub struct GithubClient {
    client: reqwest::blocking::Client,
    token: String,
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            token,
        }
    }

    /// Gets the content of a file from a github repo
    pub fn get_content(
        &self,
        username: &str,
        repository_name: &str,
        content_path: &str,
    ) -> anyhow::Result<String> {
        let response = self
            .client
            .get(format!(
                "{BASE_URL}/repos/{username}/{repository_name}/contents/{content_path}"
            ))
            .header(ACCEPT, "application/json")
            .header(USER_AGENT, "bevy-website-generate-assets")
            .bearer_auth(self.token.clone())
            .send()?;

        #[derive(Deserialize)]
        struct GithubContentResponse {
            encoding: String,
            content: String,
        }

        let json: GithubContentResponse = response.json()?;

        // The github rest api is supposed to return the content as a base64 encoded string
        if json.encoding == "base64" {
            let data = base64::decode(json.content.replace('\n', "").trim())?;
            Ok(String::from_utf8(data)?)
        } else {
            bail!("Content is not in base64");
        }
    }

    /// Gets the license from a github repo
    /// Technically, github supports multiple licenses, but the api only returns one
    pub fn get_license(&self, username: &str, repository_name: &str) -> anyhow::Result<String> {
        let response = self
            .client
            .get(format!(
                "{BASE_URL}/repos/{username}/{repository_name}/license"
            ))
            .header(ACCEPT, "application/json")
            .header(USER_AGENT, "bevy-website-generate-assets")
            .bearer_auth(self.token.clone())
            .send()?;

        #[derive(Deserialize)]
        struct GithubLicenseResponse {
            license: GithubLicenseLicense,
        }

        #[derive(Deserialize)]
        struct GithubLicenseLicense {
            spdx_id: String,
        }

        let json: GithubLicenseResponse = response.json()?;

        Ok(json.license.spdx_id)
    }
}
