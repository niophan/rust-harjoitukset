use std::io;

fn month_from_number(month: u8) -> Month {
    match month {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => Month::January,
    }
}


fn get_year_month_day(date: &str) -> (u32, u8, u8) {
    let date_string = date.trim();
    assert!(date_string.len() == 8);

    let year_string = &date_string[..4];
    let year = year_string.parse().unwrap();

    let month_string = &date_string[4..6];
    let month = month_string.parse().unwrap();

    let day_string = &date_string[6..8];
    let day = day_string.parse().unwrap();

    (year, month, day)
}

fn print_user_input_event(events: &[Event; 22], selected_month_day: &(u8, u8)) {
    let selected_month = month_from_number(selected_month_day.0);
    for day in 15..=29 {
        if day == selected_month_day.1 {
            println!("Event of  {:?} {}", selected_month, day);
            for event in events {
                let defined_day = (event.date.month, event.date.day);
                if defined_day  == (selected_month, selected_month_day.1) {
                    println!("{}: {:?}, ({:#?})", event.date.year, event.description, event.category.primary)
                }
            }
        }        
    }
}


fn user_option(events: &[Event; 22]) {
    let mut user_input = String::new();
    println!("Anna päivän määrä, esim(20260117):");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Rivin lukeminen epäonnistui");
    let selected_day: (u32, u8, u8) = get_year_month_day(&user_input);
    println!("Käyttäjä antoi päivien lukumäärän: {}-{}-{}", selected_day.0, selected_day.1, selected_day.2 );
    let selected_month_day: (u8, u8) = (selected_day.1, selected_day.2);
    match selected_month_day {
        (1, 15..=29) => print_user_input_event(&events, &selected_month_day),
        _ => println!("Virhellinen päivä/kuukausi. Päivämäärä on oltava väliltä 15.01-29.01")
    };
}

fn hardcode_option(events: &[Event; 22]) {
    let month_day = MonthDay{ month: Month::January, day: 20 };
    for day in 15..=29 {
        if day == month_day.day {
            println!("Event of  {:?} {}", month_day.month, day);
            for event in events {
                if event.month_day() == month_day {
                    println!("{}: {:?}, ({:#?})", event.date.year, event.description, event.category.primary);
                }
            }
        }
    }
}

fn _is_leap_year(year: i32) -> bool {
 (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn _day_count(month: Month, year: i32) -> u8 {
    match month {
        Month:: April | Month::June | Month::September | Month::November => 30,
        Month::February => if _is_leap_year(year) { 29 } else { 28 },
        _ => 31
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
 January,
 February,
 March,
 April,
 May,
 June,
 July,
 August,
 September,
 October,
 November,
 December,
}

struct Date {
    year: i16,
    month: Month,
    day: u8
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8
}

#[allow(dead_code)]
#[derive(Debug)]
struct Category {
 primary: String,
 secondary: Option<String>,
}

struct Event {
    date: Date,
    description: String,
    category: Category
}

impl Date {
    fn new(year: i16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string())
        }
    }
    fn _from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None
        }
    }
}

impl Event {
    fn new(date: Date, description: String, category: Category) -> Self {
        Self { date, description, category }
    }

    fn month_day(&self) -> MonthDay {
        MonthDay { month: self.date.month, day: self.date.day }
    }
}


fn main() {
    // Data
    let events = [
        Event::new(
            Date::new(1759, Month::January, 15),
            String::from("British Museum opens in Montague House, London"),
            Category::new("Culture & Heritage", "Global")),
        Event::new(
            Date::new(2001, Month::January, 15),
            String::from("Wikipedia, a free Wiki or content encyclopedia, is launched by Jimmy Wales and Larry Sanger"),
            Category::new("Technology & Internet", "Local")),
        Event::new(
            Date::new(1920, Month::January, 16),
            String::from("First assembly of the League of Nations is held in Paris"),
              Category::new("International Organizations & Diplomacy", "Global")),
        Event::new(
            Date::new(1524, Month::January, 17),
            String::from("Italian explorer Giovanni da Verrazzano's sets sail aboard French ship La Dauphine to find a passage to China - finds North America instead"),
            Category::new("Exploration & Discovery", "Global")),
        Event::new(
            Date::new(1912, Month::January, 17),
            String::from("Captain Robert Scott's expedition arrives at the South Pole, one month after Roald Amundsen"),
            Category::new("Exploration & Discovery", "Global")),
        Event::new(
            Date::new(1946, Month::January, 17),
            String::from("United Nations Security Council holds its 1st meeting, at Westminster Central Hall in Westminster, England"),
            Category::new("International Organizations & Diplomacy", "Global")),
        Event::new(
            Date::new(1960, Month::January, 18),
            String::from("US & Japan sign joint defense treaty"),
            Category::new("Defense & Security", "Global")),
        Event::new(
            Date::new(1960, Month::January, 19),
            String::from("President Eisenhower and Prime Minister Nobusuke Kishi sign the US-Japan Security Treaty"),
            Category::new("Defense & Security", "Global")),
        Event::new(
            Date::new(2009, Month::January, 20),
            String::from("Barack Obama is inaugurated as the 44th President of the United States of America, becoming the United States' first African-American president"),
            Category::new("Elections & Inaugurations", "Local")),
        Event::new(
            Date::new(2017, Month::January, 20),
            String::from("Donald Trump is inaugurated as the 45th President of the United States of America and Mike Pence as the 48th Vice President"),
            Category::new("Elections & Inaugurations", "Local")),
        Event::new(
            Date::new(2002, Month::January, 21),
            String::from("The Canadian Dollar sets all-time low against the US Dollar (US$0.6179)"),
            Category::new("Economy & Business", "Global")),
        Event::new(
            Date::new(1989, Month::January, 22),
            String::from("After winning his third Super Bowl as head coach of the San Francisco 49ers, Bill Walsh retires"),
            Category::new("Sports", "Local")),
        Event::new(
            Date::new(2018, Month::January, 22),
            String::from("US government ends three-day shutdown after an agreement in Congress to extend funding"),
            Category::new("National Politics & Governance", "Local")),
        Event::new(
            Date::new(1950, Month::January, 23),
            String::from("Israeli Knesset declares Jerusalem the capital of Israel"),
            Category::new("National Politics & Governance", "Global")),
        Event::new(
            Date::new(1977, Month::January, 23),
            String::from("Miniseries \"Roots\" premieres on ABC"),
            Category::new("Media & Entertainment", "Local")),
        Event::new(
            Date::new(1857, Month::January, 24),
            String::from("University of Calcutta is founded as the first full-fledged university in South Asia"),
            Category::new("Education", "Global")),
        Event::new(
            Date::new(1554, Month::January, 25),
            String::from("City of São Paulo founded in Brazil"),
            Category::new("Urban History & City Founding", "Global")),
        Event::new(
            Date::new(1482, Month::January, 26),
            String::from("\"Pentateuch\", the Jewish Bible, is first printed as a book in Bologna, Italy"),
            Category::new("Culture & Heritage", "Global")),
        Event::new(
            Date::new(1924, Month::January, 27),
            String::from("Vladimir Lenin is placed in a Mausoleum in Red Square, Moscow"),
            Category::new("National Politics & Governance", "Global")),
        Event::new(
            Date::new(1970, Month::January, 27),
            String::from("Movie rating system modifies the 'M' rating to 'PG'"),
            Category::new("Media & Entertainment", "")),
        Event::new(
            Date::new(1819, Month::January, 28),
            String::from("British colonial officer Stamford Raffles lands in Singapore"),
            Category::new("Exploration & Discovery", "Global")),
        Event::new(
            Date::new(1892, Month::January, 29),
            String::from("The Coca-Cola Company is incorporated in Atlanta, Georgia"),
            Category::new("Economy & Business", "Local")),
    ];
    let mut options = String::new();
    println!("Valita, mitä haluat toteuttaa: \n(1): Käyttäjäsyöte \n(2): Hardcode 20 January \n(ei mitään toimintaa)");
    io::stdin()
        .read_line(&mut options)
        .expect("Rivin lukeminen epäonnistui");
    let option_string = options.trim();
    assert_eq!(option_string.len(), 1,
        "Valintamerkkijonon pituuden on oltava täsmälleen yksi merkki");
    let option = option_string.parse().unwrap();
    match option {
        1 => user_option(&events),
        2 => hardcode_option(&events),
        _ => print!("Ei toimintaaa")
    }
}