use serde::Deserialize;
use chrono::{Datelike, Local, NaiveDate};
use std::error::Error;
use std::path::Path;

mod birthday;
pub mod events;
pub mod providers;

use events::{Event, MonthDay};
use crate::providers::EventProvider;
use crate::providers::{
    csvfile::CSVFileProvider,
  newprovider::NewProvider,
    textfile::TextFileProvider,
    sqlite::SQLiteProvider,
};

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
  providers: Vec<ProviderConfig>,
}

pub fn run(config: &Config, config_path: &Path)-> Result<(), Box<dyn Error>> {
  birthday::handle_birthday();

  let mut events: Vec<Event> = Vec::new();
  let providers = create_providers(config, config_path);
  let mut db_event_lines: Vec<String> = Vec::new();
  let mut count = 0;
  for provider in providers {
    let provider_name = provider.name();
    provider.get_events(&mut events); // polymorphism at work!
    let new_count = events.len();
    if provider_name == "events-db" {
      for event in &events[count..new_count] {
        db_event_lines.push(format!("{}", event));
      }
    }
    println!(
      "Got {} events from provider '{}'", 
      new_count - count,
      provider_name);
    count = new_count;
  }

  for line in db_event_lines {
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

// Try to create all the event providers specified in `config`.
fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> { 
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
      "new" => {
        let provider = NewProvider::new(&cfg.name);
        providers.push(Box::new(provider));
      },
      "sqlite" => {
        let provider = SQLiteProvider::new(&cfg.name, &path);
        providers.push(Box::new(provider));
      },
      _ => {
        eprintln!("Unable to make provider: {:?}", cfg);
      }
    }
  }

  providers
}