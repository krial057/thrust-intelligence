use crate::model::event::{EventFull, EventFullEmbedded, EventIdentifier};
use crate::{MispResult, MISP};
use uuid::Uuid;

pub enum DirectOrIndirectIdentifier {
    Direct(EventIdentifier),
    Indirect(),
}

/// The Request's lifetime is bound to the client's lifetime
pub struct EventRequest<'a> {
    id: EventIdentifier,
    misp_client: &'a MISP,
    cached_local: Option<EventFull>,
}

impl EventRequest<'_> {
    pub fn new<'a>(misp_client: &'a MISP, id: EventIdentifier) -> EventRequest<'a> {
        EventRequest {
            id,
            misp_client,
            cached_local: None,
        }
    }

    async fn download_to_cache(&mut self) -> MispResult<EventFull> {
        let event: EventFullEmbedded = self
            .misp_client
            .internal_api_call_get(format!("events/view/{}", self.id.to_url_id()))
            .await?;
        Ok(event.event)
    }

    async fn cached(&mut self) -> MispResult<&EventFull> {
        if self.cached_local.is_none() {
            self.cached_local = Some(self.download_to_cache().await?);
        };
        Ok(self.cached_local.as_ref().unwrap())
    }

    pub async fn retrieve(&mut self) -> MispResult<EventFull> {
        let event_ref = self.cached().await?;
        Ok(event_ref.clone())
    }

    pub async fn id(&mut self) -> MispResult<u64> {
        match self.id {
            EventIdentifier::Global(_) => Ok(self.cached().await?.id()),
            EventIdentifier::Local(id) => Ok(id),
        }
    }

    pub async fn uuid(&mut self) -> MispResult<Uuid> {
        match self.id {
            EventIdentifier::Global(uuid) => Ok(uuid),
            EventIdentifier::Local(_) => Ok(self.cached().await?.uuid()),
        }
    }
}
