mod events;
mod providers;

use events::Event;
use providers::EventProvider;
use providers::newprovider::NewProvider;

fn main() {
    let mut events: Vec<Event> = Vec::new();

    let new_provider = NewProvider::new("Rust programming");
    new_provider.get_events(&mut events);

    for event in events {
        println!("{}: {}", event.year(), event.description);
    }
}


#[cfg(test)]
mod tests {
    use crate::events::MonthDay;
    use crate::NewProvider;
    use crate::EventProvider;

    #[test]
    fn new_provider_name() {
        let provider = NewProvider::new("Rust programming");
        assert_eq!(provider.name(), "Rust programming");
    }

    #[test]
    fn new_provider_with_two_events() {
        let provider = NewProvider::new("Rust programming");
        let mut events = Vec::new();
        provider.get_events(&mut events);
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn new_provider_with_correct_rust_event() {
        let provider = NewProvider::new("Rust programming");
        let mut events = Vec::new();
        provider.get_events(&mut events);
        assert_eq!(events[1].year(), 2015);
        assert_eq!(events[1].month_day(), MonthDay { month: 5, day: 15 });
        assert_eq!(events[1].description, "Rust 1.0.0 released")
    }
}