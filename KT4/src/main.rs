use std::env;
use chrono::{Local, NaiveDate, Datelike};

fn main() {
    let key = "BIRTHDATE";
    let bdate= match env::var(key) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("{key} ei ole asettu! \nSyötä BIRTHDATE=YYYY-MM-DD(esim., BIRTHDATE=2000-08-17");
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

    if count_days > 0 {
        print!("You are {count_days} days old.");
        if count_days % 1000 == 0 {
            println!(" That's a nice, round number!");
        } 
    } else if count_days < 0 {
        println!("Are you from the future?");
    } else if count_days == 0 {
        println!("Looks like you're new here.");
    }
}