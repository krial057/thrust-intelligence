use crate::model::event::{EventFull, EventFullEmbedded};
use crate::model::organization::GenericOrganizationIdentifier;
use crate::model::serialization_helpers::option_date_to_mispdate;
use crate::{MispResult, MISP};
use chrono::{Date, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct EventListResponse {
    response: Vec<EventFullEmbedded>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SearchQuery {
    #[serde(rename = "returnFormat")]
    return_format: String,

    #[serde(rename = "org")]
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<GenericOrganizationIdentifier>,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "option_date_to_mispdate")]
    after: Option<Date<Utc>>,

    #[serde(rename = "eventinfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct EmbeddedSearchQuery {
    request: SearchQuery,
}

impl SearchQuery {
    pub fn new() -> Self {
        Self {
            return_format: "json".into(),
            organization: None,
            after: None,
            info: None,
            limit: None,
        }
    }
}
// The Request's lifetime is bound to the client's lifetime
pub struct EventListRequest<'a> {
    search_query: Option<EmbeddedSearchQuery>,
    misp_client: &'a MISP,
    cached_local: Option<Vec<EventFull>>,
}

impl EventListRequest<'_> {
    pub fn new(misp_client: &MISP, search_query: Option<EmbeddedSearchQuery>) -> EventListRequest {
        EventListRequest {
            search_query,
            misp_client,
            cached_local: None,
        }
    }

    async fn download_to_cache(&mut self) -> MispResult<Vec<EventFull>> {
        let event_list: EventListResponse = match &self.search_query {
            Some(query) => {
                self.misp_client
                    .internal_api_call_post(format!("events/restSearch"), query)
                    .await?
            }
            None => {
                self.misp_client
                    .internal_api_call_get(format!("events"))
                    .await?
            }
        };
        Ok(event_list
            .response
            .iter()
            .map(|e| e.event.clone())
            .collect())
    }

    async fn cached(&mut self) -> MispResult<&Vec<EventFull>> {
        if self.cached_local.is_none() {
            self.cached_local = Some(self.download_to_cache().await?);
        };
        Ok(self.cached_local.as_ref().unwrap())
    }

    pub async fn retrieve(&mut self) -> MispResult<Vec<EventFull>> {
        Ok(self.cached().await?.clone())
    }

    pub fn from_organization(
        &mut self,
        organization: impl Into<GenericOrganizationIdentifier>,
    ) -> &mut Self {
        let search_query = self.search_query.get_or_insert(EmbeddedSearchQuery {
            request: SearchQuery::new(),
        });
        search_query.request.organization = Some(organization.into());
        self
    }

    pub fn containing_info(&mut self, str: impl AsRef<str>) -> &mut Self {
        let search_query = self.search_query.get_or_insert(EmbeddedSearchQuery {
            request: SearchQuery::new(),
        });
        search_query.request.info = Some(format!("%{}%", str.as_ref()));
        self
    }

    pub fn with_exact_info(&mut self, str: impl Into<String>) -> &mut Self {
        let search_query = self.search_query.get_or_insert(EmbeddedSearchQuery {
            request: SearchQuery::new(),
        });
        search_query.request.info = Some(str.into());
        self
    }

    pub fn after(&mut self, date: Date<Utc>) -> &mut Self {
        let search_query = self.search_query.get_or_insert(EmbeddedSearchQuery {
            request: SearchQuery::new(),
        });
        search_query.request.after = Some(date);
        self
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        let search_query = self.search_query.get_or_insert(EmbeddedSearchQuery {
            request: SearchQuery::new(),
        });
        search_query.request.limit = Some(limit);
        self
    }
}
