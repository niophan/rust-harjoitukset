use crate::events::{Category, Event, EventKind};
use crate::EventProvider;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::BufWriter;
use crate::filters::EventFilter;
use crate::providers::EventProviderError;

#[allow(dead_code)]
pub struct CSVFileProvider {
    name: String,
    path: PathBuf,
}

#[allow(dead_code)]
impl CSVFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}
impl EventProvider for CSVFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(self.path.clone())
            .expect("existing CSV file");
        for result in reader.records() {
            let record = result.unwrap();
            let date_string = record[0].to_string();
            let description = record[1].to_string();
            let category_string = record[2].to_string();
            match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(date) => {
                    let category = Category::from_str(&category_string);
                    let event = Event::new_singular(date, description.clone(), category);
                    if filter.accepts(&event) {
                        events.push(event);
                    }
                }
                Err(_) => {
                    eprintln!("Invalid date '{}'", date_string);
                }
            }
        }
    }

    fn is_add_supported(&self) -> bool { true }

    fn add_event(&self, event: &Event) -> Result<(), EventProviderError> {
        if !self.is_add_supported() {
            return Err(super::EventProviderError::OperationNotSupported);
        }

        let file = OpenOptions::new()
            .append(true)
            .open(self.path.clone())
            .expect("path to text file for writing");

        let writer = BufWriter::new(file);
        let mut csv_writer = csv::Writer::from_writer(writer);

        let date_string = match event.kind {
            EventKind::Singular(date) => {
                date.format("%Y-%m-%d").to_string()
            }
        };

        csv_writer.write_record([
            date_string, 
            event.description.clone(), 
            format!("{}", event.category)
        ]).unwrap();
        
        csv_writer.flush().unwrap();

        Ok(())
    }
}