use std::env;
use chrono::{Local, NaiveDate, Datelike};

pub fn handle_birthday() {
  const KEY: &str = "BIRTHDATE";
  let bdate= match env::var(KEY) {
      Ok(value) => value,
      Err(_) => {
          eprintln!("{KEY} ei ole asettu! \nSyötä BIRTHDATE=YYYY-MM-DD(esim., BIRTHDATE=2000-08-17");
          std::process::exit(1);
      }
  };

  let birthdate = match NaiveDate::parse_from_str(&bdate, "%Y-%m-%d") {
      Ok(date) => date,
      Err(_) => {
          eprintln!("Virheellinen päivämäärä: '{bdate}'. Käytä muodossa  YYYY-MM-DD");
          std::process::exit(1);
      }
  };

  let now = Local::now().date_naive();
  if birthdate.month() == now.month() && birthdate.day() == now.day() {
      print!("Happy birthday! ");
  }

  let count_days = now.signed_duration_since(birthdate).num_days();  
  println!("{}", make_message(count_days));
}

fn make_message(counts_days: i64) -> String {
  if counts_days > 0 {
        if counts_days % 1000 == 0 {
            return format!("You are {counts_days} days old. That's a nice, round number!");
        }
        return format!("You are {counts_days} days old.");
    } else if counts_days < 0 {
        return String::from("Are you from the future?");
    } else {
        return String::from("Looks like you're new here.");
    }
}