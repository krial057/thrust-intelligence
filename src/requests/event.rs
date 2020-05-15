use crate::model::event::{EventFull, EventFullEmbedded, EventIdentifier};
use crate::{MispResult, MISP};
use std::cell::RefCell;
use uuid::Uuid;

pub enum DirectOrIndirectIdentifier {
    Direct(EventIdentifier),
    Indirect(),
}

/// The Request's lifetime is bound to the client's lifetime
pub struct RemoteEventRequest<'a> {
    id: EventIdentifier,
    misp_client: &'a MISP,
    cached_local: RefCell<Option<EventFull>>,
}

impl RemoteEventRequest<'_> {
    pub fn new<'a>(misp_client: &'a MISP, id: EventIdentifier) -> RemoteEventRequest<'a> {
        RemoteEventRequest {
            id,
            misp_client,
            cached_local: RefCell::new(None),
        }
    }

    async fn download_to_cache(&self) -> MispResult<EventFull> {
        let embedded_event: EventFullEmbedded = self
            .misp_client
            .internal_api_call(format!("events/view/{}", self.id.to_url_id()))
            .await?;
        Ok(embedded_event.event)
    }

    async fn cache_if_needed(&self) -> MispResult<()> {
        if self.cached_local.borrow().is_none() {
            *self.cached_local.borrow_mut() = Some(self.download_to_cache().await?);
        };
        Ok(())
    }

    pub async fn get(&self) -> MispResult<EventFull> {
        self.cache_if_needed().await?;
        let event_ref = self.cached_local.borrow();
        Ok(event_ref.as_ref().unwrap().clone())
    }

    pub async fn id(&self) -> MispResult<u64> {
        match self.id {
            EventIdentifier::Global(_) => {
                self.cache_if_needed().await?;
                let event_ref = self.cached_local.borrow();
                Ok(event_ref.as_ref().unwrap().id())
            }
            EventIdentifier::Local(id) => Ok(id),
        }
    }

    pub async fn uuid(&self) -> MispResult<Uuid> {
        match self.id {
            EventIdentifier::Global(uuid) => Ok(uuid),
            EventIdentifier::Local(_) => {
                self.cache_if_needed().await?;
                let event_ref = self.cached_local.borrow();
                Ok(event_ref.as_ref().unwrap().uuid())
            }
        }
    }
}
