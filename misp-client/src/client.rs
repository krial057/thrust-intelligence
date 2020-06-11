use surf::http_types::headers::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use url::Url;

use crate::error::MispResult;
use crate::requests::api::EventsApi;
use misp_types::server_info::ServerInfo;

#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;
#[cfg(feature = "serde")]
use serde::Serialize;

/// This is the starting point: A MISP client. It us used to connect to a MISP Server.
///
/// # Examples
///
/// ```no_run
/// # use misp_client::{MISP, MispResult};
/// # #[async_std::main]
/// # async fn main() -> MispResult<()>  {
///     let misp = MISP::new("https://misp.demo.com", "VERYSECRETTOKEN");
///
///     // Receive 3 latest event from Misp server
///     let events = misp
///         .events()
///         .list()
///         .limit(3)
///         .retrieve()
///         .await?;
///     # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MISP {
    base_url: Url,
    auth_token: String,
}

impl MISP {
    /// Creates a new MISP Client given a base URL and an authorization token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use misp_client::MISP;
    /// let misp = MISP::new("https://misp.demo.com", "VERYSECRETTOKEN");
    /// ```
    /// As always, make sure to never store the secret token in the source code. It's not a good practice as
    /// it could be leaked when the source code is published somewhere. A better approach is to store the secret
    /// in an environment variable:
    /// ```no_run
    /// # use std::env;
    /// # use misp_client::MISP;
    /// let base_url =
    ///    env::var("MISP_ROOT_URL").expect("Please set the MISP_ROOT_URL environment variable");
    /// let auth_token =
    ///    env::var("MISP_AUTH_TOKEN").expect("Please set the MISP_AUTH_TOKEN environment variable");
    ///
    /// let misp = MISP::new(base_url, auth_token);
    /// ```
    pub fn new(base_url: impl AsRef<str>, auth_token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.as_ref().parse().unwrap(),
            auth_token: auth_token.into(),
        }
    }

    pub(crate) async fn internal_api_call_get<T: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str>,
    ) -> MispResult<T> {
        let endpoint_url = self.base_url.join(endpoint.as_ref())?;
        let body_bytes = surf::get(endpoint_url)
            .set_header(AUTHORIZATION, self.auth_token.as_str())
            .set_header(ACCEPT, "application/json")
            .set_header(CONTENT_TYPE, "application/json")
            .set_header(USER_AGENT, "rs_misp")
            .recv_bytes()
            .await?;
        println!("{}", String::from_utf8(body_bytes.clone()).unwrap());
        Ok(serde_json::from_slice::<T>(&body_bytes)?)
    }

    pub(crate) async fn internal_api_call_post<T: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str>,
        json: &impl Serialize,
    ) -> MispResult<T> {
        /*
                println!(
                    "Sending json: {:?}",
                    serde_json::to_string_pretty(json).unwrap()
                );
        */
        let endpoint_url = self.base_url.join(endpoint.as_ref())?;
        let body_bytes = surf::post(endpoint_url)
            .set_header(AUTHORIZATION, self.auth_token.as_str())
            .set_header(ACCEPT, "application/json")
            .set_header(CONTENT_TYPE, "application/json")
            .set_header(USER_AGENT, "rs_misp")
            .body_json(json)?
            .recv_bytes()
            .await?;
        Ok(serde_json::from_slice::<T>(&body_bytes)?)
    }

    pub async fn server_info(&self) -> MispResult<ServerInfo> {
        Ok(self
            .internal_api_call_get("servers/getVersion.json")
            .await?)
    }

    pub fn events(&self) -> EventsApi<'_> {
        EventsApi::new(self)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn create_client() {
        // Basic test
        let misp = MISP::new("https://test.xyz/", "12345678");
        assert_eq!(misp.base_url.to_string(), "https://test.xyz/");
        assert_eq!(misp.auth_token, "12345678");

        // Should append a slash at the end of a root url
        let misp = MISP::new("https://test.xyz", "12345678");
        assert_eq!(misp.base_url.to_string(), "https://test.xyz/");
        assert_eq!(misp.auth_token, "12345678");
    }
}
