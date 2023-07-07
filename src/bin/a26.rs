// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::Utc;
use chrono::TimeZone;
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let time_now = SystemTime::now();
    let duration = time_now.duration_since(UNIX_EPOCH).expect("Failed to get duration");
    let seconds_since_epoch = duration.as_secs();
    let formatted_datetime = Utc.timestamp_opt(seconds_since_epoch.try_into().unwrap(), 0).unwrap();

    println!("{:?}", formatted_datetime.to_rfc2822());
}
