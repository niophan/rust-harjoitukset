use std::fs;
use std::path::PathBuf;
use today::{run, Config};
use clap::Parser;
use chrono::{NaiveDate, Local, Datelike};
use today::events::MonthDay;
use today::filters::{EventFilter, FilterBuilder};

#[derive(Parser)]
#[command(name = "today")]
struct Args {
 #[arg(short, long, help = "Event date in MMDD format")]
 date: Option<String>,
}


fn main() {
    let args = Args::parse();

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    } else {
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };

    let filter: EventFilter = FilterBuilder::new()
        .month_day(month_day)
        .build();

    
    const APP_NAME: &str = "today";
    if let Some(config_path) = get_config_path(APP_NAME) {
        let toml_path = config_path.join(format!("{}.toml", APP_NAME));
        println!("Looking for configuration file '{}'", &toml_path.display());
        let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
        let config: Config = toml::from_str(&config_str).expect("valid configuration file");
        println!("config: {:#?}", config);
        run(&config, &config_path, &filter).expect("run should succeed");
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
    use today::events::MonthDay;
    use today::filters::EventFilter;
    use today::providers::simpleprovider::SimpleProvider;
    use today::providers::EventProvider;

    #[test]
    fn new_provider_name() {
        let provider = SimpleProvider::new("Rust programming");
        assert_eq!(provider.name(), "Rust programming");
    }

    #[test]
    fn new_provider_with_two_events() {
        let provider = SimpleProvider::new("Rust programming");
        let mut events = Vec::new();
        provider.get_events(&EventFilter::new(), &mut events);
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn new_provider_with_correct_rust_event() {
        let provider = SimpleProvider::new("Rust programming");
        let mut events = Vec::new();
        provider.get_events(&EventFilter::new(), &mut events);
        assert_eq!(events[1].year(), 2015);
        assert_eq!(events[1].month_day(), MonthDay { month: 5, day: 15 });
        assert_eq!(events[1].description, "Rust 1.0.0 released")
    }
}