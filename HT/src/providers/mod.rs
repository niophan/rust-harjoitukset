use crate::events::Event;
use crate::filters::EventFilter;

pub mod csvfile;
pub mod simpleprovider;
pub mod sqlite;
pub mod textfile;
pub mod web;

#[allow(dead_code)]
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
    fn is_add_supported(&self) -> bool { false }
    fn add_event(&self, event: &Event) -> Result<(), EventProviderError>;
}


pub enum EventProviderError {
    OperationNotSupported,
    OperationFailed,
}