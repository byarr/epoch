use std::env::args_os;
use std::process::exit;
use chrono::{Utc, TimeZone, DateTime, Local};
use crate::Units::{NANO, MILLI, SECONDS, MICRO};
use std::fmt::{Display, Formatter};

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

            println!("Assuming {}", dt.unit);
            println!("UTC:   {}", dt.dt);

            let as_local = dt.dt.with_timezone(&Local);
            println!("Local: {}", as_local);
        }
    }
}

enum Units {
    SECONDS,
    MILLI,
    MICRO,
    NANO,
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SECONDS => {write!(f, "seconds")}
            MILLI => {write!(f, "milli-seconds")}
            MICRO => {write!(f, "micro-seconds")}
            NANO => {write!(f, "nano-seconds")}
        }
    }
}

impl Units {
    fn per_second(&self) -> i64 {
        match self {
            Units::SECONDS => {1}
            Units::MILLI => {1_000}
            Units::MICRO => {1_000_000}
            Units::NANO => {1_000_000_000}
        }
    }
    fn to_date_time(&self, i: i64) -> DateTime<Utc> {
        let seconds = i / self.per_second();

        let remaining = i - (seconds * self.per_second());
        let nanos = remaining * ( NANO.per_second() / self.per_second());
        Utc.timestamp(seconds, nanos as u32)
    }
}

struct ParsedTime {
    unit: Units,
    dt: DateTime<Utc>,
}

fn try_parse(input: String) -> Option<ParsedTime> {
    let int_result: Result<i64, _> = input.parse();
    if let Ok(epoch) = int_result {
        Some(int_to_datetime(epoch))
    } else {
        None
    }
}

fn int_to_datetime(i: i64) -> ParsedTime {
    let unit =  if i < 10_000_000_000 {
        SECONDS
    } else if i < 10_000_000_000_000 {
        MILLI
    } else if i < 10_000_000_000_000_000 {
        MICRO
    } else {
        NANO
    };
    let dt = unit.to_date_time(i);
    ParsedTime {
        unit,
        dt,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_to_datetime_seconds() {
        assert_eq!(int_to_datetime(1630779114).dt, Utc.ymd(2021, 9, 4).and_hms(18, 11, 54));
    }

    #[test]
    fn test_int_to_datetime_milliseconds() {
        assert_eq!(int_to_datetime(1630779114123).dt, Utc.ymd(2021, 9, 4).and_hms_milli(18, 11, 54, 123));
    }

    #[test]
    fn test_int_to_datetime_microseconds() {
        assert_eq!(int_to_datetime(1630779114123456).dt, Utc.ymd(2021, 9, 4).and_hms_micro(18, 11, 54, 123456));
    }

    #[test]
    fn test_int_to_datetime_nanoseconds() {
        assert_eq!(int_to_datetime(1630779114123456789).dt, Utc.ymd(2021, 9, 4).and_hms_nano(18, 11, 54, 123456789));
    }
}