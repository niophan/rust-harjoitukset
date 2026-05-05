use std::collections::HashSet;

use crate::events::{Event, MonthDay, Category};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    options: HashSet<FilterOption>,
    excludes: HashSet<Category>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
            excludes: HashSet::new(),
        }
    }

    pub fn accepts(&self, event: &Event) -> bool {
        if self.excludes.contains(&event.category()) {
            return false;
        }
        
        if self.options.is_empty() {
            return true;
        }

        let mut results: Vec<bool> = Vec::new();

        for option in self.options.iter() {
            let result = match option {
                FilterOption::MonthDay(month_day) => {
                    *month_day == event.month_day()
                },
                FilterOption::Category(category) => {
                    *category == event.category()
                },
                FilterOption::Text(text) => {
                    event.description().contains(text)
                }
            };
            results.push(result);
        }

        results.iter().all(|&option| option)
    }
}

pub struct FilterBuilder {
    options: HashSet<FilterOption>,
    excludes: HashSet<Category>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
            excludes: HashSet::new(),
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        self.options.insert(FilterOption::MonthDay(month_day));
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.options.insert(FilterOption::Category(category));
        self
    }

    pub fn text(mut self, text: String) -> FilterBuilder {
        self.options.insert(FilterOption::Text(text));
        self
    }

    pub fn exclude_category(mut self, category: Category) -> FilterBuilder {
        self.excludes.insert(category);
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            options: self.options,
            excludes: self.excludes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate};

    #[test]
    fn filter_accepts_anything() {
      let rust_category = Category::new("programming", "rust");
      let event = Event::new_singular(
        NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
        "Rust 1.94.0 released".to_string(),
        rust_category.clone());
      let filter = FilterBuilder::new()
        .build();
      assert!(filter.accepts(&event));
    }

   #[test]
    fn filter_with_criteria_accepts_only_true_values_return() {
      let category = Category::new("Loppukoe", "Api-Kehitys");
      let event = Event::new_singular(
        NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
        "tärkeä".to_string(),
        category.clone());
      let filter = FilterBuilder::new()
        .month_day(MonthDay::new(3, 25))
        .category(Category::new("Loppukoe", "Api-Kehitys"))
        .text("tärkeä".to_string())
        .build();
      assert!(filter.accepts(&event));
    }

   #[test]
    fn filter_with_criteria_reject_if_one_false_value_return() {
      let category = Category::from_primary("birthday");
      let event = Event::new_singular(
        NaiveDate::from_ymd_opt(2026, 3, 25).unwrap(),
        "call Alex".to_string(),
        category.clone());
      let filter = FilterBuilder::new()
        .month_day(MonthDay::new(3, 25))
        .category(Category::from_primary("birthday"))
        .text("call Alexi".to_string())
        .build();
      assert!(!filter.accepts(&event));
    }
}

