use std::ops::Sub;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn start() {
    // Duration example ----------------------------------------------------------------------------
    let five_seconds = Duration::new(5, 0);
    let five_seconds = Duration::from_secs(5);
    let ten_millis = Duration::from_millis(10);
    // print
    println!("Five seconds: {:?}", five_seconds);
    println!("Ten milliseconds: {:?}", ten_millis);

    // calculate duration
    let start = Duration::from_millis(2000);
    let end = Duration::from_millis(5000);
    let duration = end.sub(start);
    println!("Duration  of {:?} {:?}: {:?}", start, end, duration);

    // check sub
    let start_01 = Duration::from_millis(20000);
    let end_01 = Duration::from_millis(5000);
    // this has `checked_sub`, `checked_add`, `checked_mul`, `checked_div` methods
    let duration_01 = end_01.checked_sub(start_01);
    match duration_01 {
        Some(duration) => println!("Checked sub duration: {:?}", duration),
        None => println!("Cannot subtract {:?} from {:?}", end_01, start_01),
    }
}
