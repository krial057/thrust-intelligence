use crate::requests::event::EventRequest;
use crate::requests::event_list::EventListRequest;
use crate::MISP;
use misp_types::event::GenericEventIdentifier;

pub struct EventsApi<'a> {
    misp_client: &'a MISP,
}

/// EventsApi is bound to the lifetime of the MISP client instance
impl<'a> EventsApi<'a> {
    pub fn new(misp_client: &'a MISP) -> EventsApi<'a> {
        EventsApi { misp_client }
    }

    pub fn list(&self) -> EventListRequest<'_> {
        EventListRequest::new(self.misp_client, None)
    }

    pub fn get(&self, event: impl Into<GenericEventIdentifier>) -> EventRequest<'a> {
        EventRequest::new(self.misp_client, event.into())
    }

    pub fn add() {}

    pub fn update() {}
}
