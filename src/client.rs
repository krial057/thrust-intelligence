use serde::de::DeserializeOwned;
use surf::http_types::headers::{HeaderName, CONTENT_TYPE};
use url::Url;

use crate::error::MispResult;
use crate::model::event::EventIdentifier;
use crate::model::server_info::ServerInfo;
use crate::requests::event::RemoteEventRequest;

/// A MISP client. Used to connect to a MISP Server.
///
/// # Examples
///
/// ```no_run
/// ```
#[derive(Debug, Clone)]
pub struct MISP {
    base_url: Url,
    auth_token: String,
}

impl MISP {
    /// Creates a new MISP Client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// ```
    pub fn new(base_url: impl AsRef<str>, auth_token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.as_ref().parse().unwrap(),
            auth_token: auth_token.into(),
        }
    }

    pub(crate) async fn internal_api_call<T: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str>,
    ) -> MispResult<T> {
        // TODO replace when https://github.com/http-rs/http-types/pull/107 merges into surf
        let authorization_header_name: HeaderName = "authorization".parse().unwrap();
        let user_agent_header_name: HeaderName = "user-agent".parse().unwrap();
        let accept_header_name: HeaderName = "accept".parse().unwrap();

        let endpoint_url = self.base_url.join(endpoint.as_ref())?;
        let body_bytes = surf::get(endpoint_url)
            .set_header(authorization_header_name, &self.auth_token)
            .set_header(accept_header_name, "application/json")
            .set_header(CONTENT_TYPE, "application/json")
            .set_header(user_agent_header_name, "rs_misp")
            .recv_bytes()
            .await?;
        println!("{}", String::from_utf8(body_bytes.clone()).unwrap());
        Ok(serde_json::from_slice::<T>(&body_bytes)?)
    }

    pub async fn server_version(&self) -> MispResult<ServerInfo> {
        Ok(self.internal_api_call("servers/getVersion.json").await?)
    }

    pub async fn get_events(&self) -> MispResult<String> {
        Ok(self.internal_api_call("events").await?)
    }

    pub fn event(&self, event: impl Into<EventIdentifier>) -> RemoteEventRequest<'_> {
        RemoteEventRequest::new(self, event.into())
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
