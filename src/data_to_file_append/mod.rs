use std::fs::{self, File, OpenOptions};
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    Utc,
};
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn main() -> (DelayedFormat<StrftimeItems<'static>>, File) {
    if !fs::metadata("data.txt").is_ok() {
        // Create the file if it does not exist
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("data.txt")
            .expect("cannot create file");
    }
    let mut data_appending = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");
    let timestamp: DelayedFormat<chrono::format::StrftimeItems<'_>> =
        Utc::now().format("\n Date: %Y-%m-%d, Time:%H:%M:%S \n  ");

    (timestamp, data_appending)
}
