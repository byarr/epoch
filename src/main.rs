use std::env::args_os;
use std::process::exit;
use chrono::{Utc, TimeZone, DateTime, Local};

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
    if i < 10_000_000_000 {
        Utc.timestamp(i, 0)
    } else if i < 10_000_000_000_000 {
        //assume epoch millis
        let seconds = i / 1_000;
        let millis = i - seconds * 1000;
        let nanos = millis * 1_000_000;
        Utc.timestamp(seconds, nanos as u32)
    } else if i < 10_000_000_000_000_000 {
        // micro
        let seconds = i / 1_000_000;
        let micros = i - seconds * 1_000_000;
        let nanos = micros * 1_000;
        Utc.timestamp(seconds, nanos as u32)
    } else {
        //nano
        let seconds = i / 1_000_000_000;
        let nanos = i - seconds * 1_000_000_000;
        Utc.timestamp(seconds, nanos as u32)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_to_datetime_seconds() {
        assert_eq!(int_to_datetime(1630779114), Utc.ymd(2021, 9, 4).and_hms(18, 11, 54));
    }

    #[test]
    fn test_int_to_datetime_milliseconds() {
        assert_eq!(int_to_datetime(1630779114123), Utc.ymd(2021, 9, 4).and_hms_milli(18, 11, 54, 123));
    }

    #[test]
    fn test_int_to_datetime_microseconds() {
        assert_eq!(int_to_datetime(1630779114123456), Utc.ymd(2021, 9, 4).and_hms_micro(18, 11, 54, 123456));
    }

    #[test]
    fn test_int_to_datetime_nanoseconds() {
        assert_eq!(int_to_datetime(1630779114123456789), Utc.ymd(2021, 9, 4).and_hms_nano(18, 11, 54, 123456789));
    }
}