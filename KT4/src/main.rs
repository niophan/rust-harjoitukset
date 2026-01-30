use std::io;
use chrono::{Local, NaiveDate};


fn get_year_month_day(date: &str) -> (i32, u32, u32) {
    let date_string: String = date.chars().filter(|c| c.is_ascii_digit()).collect();
    assert_eq!(date_string.len(), 8,
        "Valintamerkkijonon pituuden on oltava täsmälleen 8 merkki");
    let year_string = &date_string[..4];
    let year = year_string.parse().unwrap();

    let month_string = &date_string[4..6];
    let month = month_string.parse().unwrap();

    let day_string = &date_string[6..8];
    let day = day_string.parse().unwrap();

    (year, month, day)
}

fn main() {
    let mut user_input: String = String::new();
    println!("Anna sinun syntymäpäivä YYYY-MM-DD: (esim 2000-07-09)");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Rivin lukeminen epäonnistui");
    let user_input = user_input.trim();
    if user_input.is_empty() {
        panic!("BIRTHDATE ei ole asetettu! \nSyötä syntymäpäivä uudelleen (esim. 2000-07-09 tai 20000709)");
    }
    let bday_string = get_year_month_day(user_input);
    let modified_bday = NaiveDate::from_ymd_opt(bday_string.0, bday_string.1, bday_string.2)
        .expect("Virheellinen päivämäärä, käytä muodossa YYYY-MM-DD (esim: 2000-07-09 tai 20000709)");
    let now = Local::now().date_naive();
    let current_date = Local::now().format("%m%d");
    let birth_date = modified_bday.format("%m%d");
    let count_days = now.signed_duration_since(modified_bday).num_days();

    if birth_date.to_string() == current_date.to_string() {
        print!("Happy Birthday! ")
    }
    if count_days > 0 {
        print!("You are {count_days} days old. ");
        if count_days % 1000 == 0{
            println!("That's a nice, round number!")
        }
    } else if count_days < 0 {
        println!("Are you from the future?")
    } else if count_days == 0{
        println!("Looks like you're new here.")
    }

}
