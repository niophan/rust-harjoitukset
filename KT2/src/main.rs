use std::io;

fn get_year_month_day(date: &str) -> (i32, i32, i32) {
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

fn print_all_events_in_week(events: [(i32, &str); 13]) {
    for event in events {
        println!("{}: {:?}", event.0, event.1)
    }
}

fn print_all_events_by_day_header(events: [(i32, &str); 13]) {
    for day in 15..=22 {
        println!("{}.1", day);
    
        for event in events {
            let event_date = event.0.to_string();
            let event_date_parts = get_year_month_day(&event_date);
            if day == event_date_parts.2 {
                println!("- {}: {:?}", event_date_parts.0, event.1);   
            }
        }
    }
}

fn main() {
    // Data
    let events = [
        (1759_01_15, "British Museum opens in Montague House, London"),
        (2001_01_15, "Wikipedia, a free Wiki or content encyclopedia, is launched by Jimmy Wales and Larry Sanger"),
        (1920_01_16, "First assembly of the League of Nations is held in Paris"),
        (1524_01_17, "Italian explorer Giovanni da Verrazzano's sets sail aboard French ship La Dauphine to find a passage to China - finds North America instead"),
        (1912_01_17, "Captain Robert Scott's expedition arrives at the South Pole, one month after Roald Amundsen"),
        (1946_01_17, "United Nations Security Council holds its 1st meeting, at Westminster Central Hall in Westminster, England"),
        (1960_01_18, "US & Japan sign joint defense treaty"),
        (1960_01_19, "President Eisenhower and Prime Minister Nobusuke Kishi sign the US-Japan Security Treaty"),
        (2009_01_20, "Barack Obama is inaugurated as the 44th President of the United States of America, becoming the United States' first African-American presiden"),
        (2017_01_20, "Donald Trump is inaugurated as the 45th President of the United States of America and Mike Pence as the 48th Vice President"),
        (2002_01_21, "The Canadian Dollar sets all-time low against the US Dollar (US$0.6179)"),
        (1989_01_22, "After winning his third Super Bowl as head coach of the San Francisco 49ers, Bill Walsh retires"),
        (2018_01_22, "US government ends three-day shutdown after an agreement in Congress to extend funding"),
    ];
    let mut user_input = String::new();

    println!("Anna päivän määrä, esim(20260117):");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Rivin lukeminen epäonnistui");
    let today = get_year_month_day(&user_input);
    let month_day = (today.1, today.2);

    // Tähän ei päde yhtään validointia
    match month_day {
        (1, 15..=22) => print_all_events_in_week(events),
        _ => println!("Virhellinen päivä. Päivän on oltava väliltä 15.1-22.1")
    };
    println!("----------------------------------------------------------------------------------");
    match month_day {
        (1, 15..=22) => print_all_events_by_day_header(events),
        _ => println!("")
    }
}