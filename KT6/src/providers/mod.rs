use crate::events::Event;

pub mod newprovider;

#[allow(dead_code)]
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}
