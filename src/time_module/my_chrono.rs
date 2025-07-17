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
}
