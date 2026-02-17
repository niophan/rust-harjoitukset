use std::fmt;
use chrono::{NaiveDate, Datelike};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    fn _new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
    fn _from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = month_string.parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
}

#[derive(Debug)]
struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
     Self {
        primary: primary.to_string(),
        secondary: Some(secondary.to_string())
     }
    }

    fn from_primary(primary: &str) -> Self {
     Self {
        primary: primary.to_string(),
        secondary: None
     }
    }

    fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split("/").collect();
        if parts.len() < 2 {
            Category { primary: parts[0].to_string(), secondary: None }
        } else {
            Category { primary: parts[0].to_string(), secondary: Some(parts[1].to_string()) }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}


#[derive(Debug)]
enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category
        }
    }

    fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }


    fn _month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) =>
                MonthDay { month: date.month(), day: date.day() },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})",
            match &self.kind {
                EventKind::Singular(date) => date.year(),
            },
            self.description, self.category)
    }
}

fn main() {
    let mut events: Vec<Event> = Vec::new();
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        Category::from_str("programming/rust")));
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        Category::new("programming", "rust")));

    for event in events {
        println!("{}: {}", event.year(), event.description);
    }
}

#[cfg(test)]
mod tests {
    use crate::Category;
    #[test]
    fn primary_and_secondary_category() {
        let category = Category::new("programming", "java");
        assert_eq!(category.primary, "programming");
        assert_eq!(category.secondary, Some("java".to_string()));
    }

    #[test]
    fn only_primary_category() {
        let category = Category::from_primary("swimming");
        assert_eq!(category.primary, "swimming");
        assert_eq!(category.secondary, None);
    }

    #[test]
    fn with_slash_separator() {
        let category = Category::from_str("food/beverage");
        assert_eq!(category.primary, "food");
        assert_eq!(category.secondary, Some("beverage".to_string()));
    }

    #[test]
    fn without_slash_separator() {
        let category = Category::from_str("city");
        assert_eq!(category.primary, "city");
        assert_eq!(category.secondary, None);
    }
}