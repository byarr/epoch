use std::env::args_os;
use std::process::exit;
use std::convert::{TryFrom, Infallible, TryInto};
use chrono::{Utc, TimeZone, DateTime, Local};
use std::error::Error;

fn main() {
    let args: Vec<_> = args_os().collect();

    if args.len() != 2 {
        eprintln!("Expected 1 argument got {}", args.len() - 1);
        exit(1);
    }

    let input = std::env::args().nth(1).expect("No time given");
    let option = try_parse(input);

    match option {
        None => { println!("Failed to parse argument")}
        Some(dt) => {
            println!("UTC:   {}", dt);

            let as_local = dt.with_timezone(&Local);
            println!("Local: {}", as_local);

        }
    }



}

fn try_parse(input: String) -> Option<DateTime<Utc>> {
    let int_result: Result<i64, _> = input.parse();
    if let Ok(epoch) = int_result {
        Some(int_to_datetime(epoch))
    } else {
        None
    }
}

fn int_to_datetime(i: i64) -> DateTime<Utc> {
    if i < 10000000000 {
        Utc.timestamp(i, 0)
    } else {
        //assume epoch millis
        let seconds = i / 1000;
        let millis = (i - seconds * 1000);
        let nanos = millis * 1000000;
        Utc.timestamp(seconds, nanos as u32)
    }
}