extern crate chrono;

pub fn start() {
    // dateTime example using chrono
    let now = chrono::Local::now();
    println!("Current date and time: {}", now);

    // UTC dateTime example using chrono
    let utc_now = chrono::Utc::now();
    println!("Current date and time in UTC: {}", utc_now);
    // Formatting date and time to a string
    println!(
        "Formatted date and time Local: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "Formatted date and time UTC: {}",
        utc_now.format("%Y-%m-%d %H:%M:%S")
    );

    // Naive dateTime example using chrono
    let naive_date = chrono::NaiveDate::from_ymd_opt(2023, 10, 1);
    match naive_date {
        Some(date) => {
            println!("Naive date: {}", date);
            // Formatting naive date to a string
            println!("Formatted naive date: {}", date.format("%Y-%m-%d"));
        }
        None => {
            println!("Invalid date");
        }
    }

    // is leap year example using chrono
    let is_leap_year = chrono::NaiveDate::from_yo_opt(2020, 366);
    match is_leap_year {
        Some(date) => {
            println!("{} is a leap year", date);
        }
        None => {
            println!("Invalid year");
        }
    }
}
