use crate::events::Event;
use crate::filters::EventFilter;

pub mod csvfile;
pub mod simpleprovider;
pub mod sqlite;
pub mod textfile;

#[allow(dead_code)]
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
}
