use serde::Deserialize;
use chrono::{Datelike, Local, NaiveDate};
use std::error::Error;
use std::path::Path;

mod birthday;
pub mod events;
pub mod filters;
pub mod providers;

use events::{Event, MonthDay};
use crate::providers::EventProvider;
use crate::providers::{
    csvfile::CSVFileProvider,
    textfile::TextFileProvider,
    sqlite::SQLiteProvider,
    web::WebProvider,
};
use crate::filters::EventFilter;


#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
  pub providers: Vec<ProviderConfig>,
}

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter, no_birthday: bool)-> Result<(), Box<dyn Error>> {
  if !no_birthday {
    birthday::handle_birthday();
  }

  let mut events: Vec<Event> = Vec::new();
  let providers = create_providers(config, config_path);

 
  let mut event_lines: Vec<String> = Vec::new();
  let mut count = 0;
  for provider in providers {
    let provider_name = provider.name();
    provider.get_events(&filter, &mut events); // polymorphism at work!
    let new_count = events.len();
    if provider_name == "events-db" {
      for event in &events[count..new_count] {
        event_lines.push(format!("{}", event));
      }
    }
    println!(
      "Got {} events from provider '{}'", 
      new_count - count,
      provider_name);
    count = new_count;
  }

  for line in event_lines {
    println!("{}", line);
  }

  let today: NaiveDate = Local::now().date_naive();
  let today_month_day = MonthDay::new(today.month(), today.day());

  for event in events {
    if today_month_day == event.month_day() {
      println!("{}", event);
    }
  }

  Ok(())
}


pub fn add_event(config: &Config, config_path: &Path, provider_name: &str, event: &Event) {
    let providers = create_providers(config, config_path);

    // Find provider by name
    let mut provider: Option<&dyn EventProvider> = None;
    for p in &providers {
        if p.name() == provider_name {
            provider = Some(p.as_ref());
            break;
        }
    }

    match provider {
        Some(p) => {
            if p.is_add_supported() {
                let _ = p.add_event(event);
            } else {
                eprintln!("Adding events is not supported for provider '{}'", p.name());
            }
        },
        None => {
            eprintln!("Unknown event provider '{}'", provider_name);
        }
    }
}


// Try to create all the event providers specified in `config`.
pub fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> { 
 // Put them in a vector of trait objects.
  let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
  for cfg in config.providers.iter() {
    let path = config_path.join(&cfg.resource);
    match cfg.kind.as_str() {
      "text" => {
        let provider = TextFileProvider::new(&cfg.name, &path);
        providers.push(Box::new(provider));
      },
      "csv" => {
        let provider = CSVFileProvider::new(&cfg.name, &path);
        providers.push(Box::new(provider));
      },
      "sqlite" => {
        let provider = SQLiteProvider::new(&cfg.name, &path);
        providers.push(Box::new(provider));
      },
      "web" => {
        let provider = WebProvider::new(&cfg.name, &cfg.resource);
        providers.push(Box::new(provider));
      },
      _ => {
        eprintln!("Unable to make provider: {:?}", cfg);
      }
    }
  }

  providers
}