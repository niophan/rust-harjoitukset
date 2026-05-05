use crate::EventProvider;
use chrono::NaiveDate;
use crate::events::{Event, Category};
use crate::filters::EventFilter;
use crate::providers::EventProviderError;

#[allow(dead_code)]
pub struct SimpleProvider {
    name: String,
}

#[allow(dead_code)]
impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
            String::from("Rust 1.92.0 released"),
            Category::from_str("programming/rust")
        );
        if filter.accepts(&event) {
            events.push(event);
        }
        
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
            String::from("Rust 1.0.0 released"),
            Category::new("programming", "rust")
        );
        if filter.accepts(&event) {
            events.push(event);
        }
    }

    fn add_event(&self, _event: &Event) -> Result<(), EventProviderError> {
        Err(EventProviderError::OperationNotSupported)
    }
}
