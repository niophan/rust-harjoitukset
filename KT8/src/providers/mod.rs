use crate::events::Event;

pub mod csvfile;
pub mod newprovider;
pub mod sqlite;
pub mod textfile;

#[allow(dead_code)]
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}
