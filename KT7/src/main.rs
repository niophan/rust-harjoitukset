mod events;
mod providers;

use std::fs;
use std::path::PathBuf;
use events::Event;
use providers::EventProvider;
use providers::newprovider::NewProvider;
use today::{run, Config};


fn main() {
    let mut events: Vec<Event> = Vec::new();

    const APP_NAME: &str = "today";
    if let Some(config_path) = get_config_path(APP_NAME) {
        let toml_path = config_path.join(format!("{}.toml", APP_NAME));
        println!("Looking for configuration file '{}'", &toml_path.display());
        let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
        let config: Config = toml::from_str(&config_str).expect("valid configuration file");
        println!("config: {:#?}", config);
        run(&config, &config_path).expect("run should succeed");
    }
}


// Gets the configuration directory path for the application
// named in the `app_name` argument.
// If the directory does not exist, tries to create it.
// Returns an optional `PathBuf` containing the directory path,
// or None if the directory can't be created.
fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        println!("Config directory: '{}'", config_dir.display());

        // Check if our config directory exists
        let config_path = config_dir.join(app_name);
        print!("App config directory: '{}'", config_path.display());

        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            } else {
                print!(" - created");
            }
        } else {
            print!(" - exists");
        }
        println!();

        return Some(config_path);
    }

    None
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