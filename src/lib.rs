use crate::Units::{Micro, Milli, Nano, Seconds};
use chrono::{DateTime, TimeZone, Utc};
use std::fmt::{Display, Formatter};

pub enum Units {
    Seconds,
    Milli,
    Micro,
    Nano,
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Seconds => {
                write!(f, "seconds")
            }
            Milli => {
                write!(f, "milli-seconds")
            }
            Micro => {
                write!(f, "micro-seconds")
            }
            Nano => {
                write!(f, "nano-seconds")
            }
        }
    }
}

impl Units {
    fn per_second(&self) -> i64 {
        match self {
            Units::Seconds => 1,
            Units::Milli => 1_000,
            Units::Micro => 1_000_000,
            Units::Nano => 1_000_000_000,
        }
    }
    fn to_date_time(&self, i: i64) -> DateTime<Utc> {
        let seconds = i / self.per_second();

        let remaining = i - (seconds * self.per_second());
        let nanos = remaining * (Nano.per_second() / self.per_second());
        Utc.timestamp(seconds, nanos as u32)
    }
}

pub struct ParsedTime {
    pub unit: Units,
    pub dt: DateTime<Utc>,
}

pub fn try_parse(input: &str) -> Option<ParsedTime> {
    let int_result: Result<i64, _> = input.parse();
    if let Ok(epoch) = int_result {
        Some(int_to_datetime(epoch))
    } else {
        None
    }
}

fn int_to_datetime(i: i64) -> ParsedTime {
    let unit = if i < 10_000_000_000 {
        Seconds
    } else if i < 10_000_000_000_000 {
        Milli
    } else if i < 10_000_000_000_000_000 {
        Micro
    } else {
        Nano
    };
    let dt = unit.to_date_time(i);
    ParsedTime { unit, dt }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_to_datetime_seconds() {
        assert_eq!(
            int_to_datetime(1630779114).dt,
            Utc.ymd(2021, 9, 4).and_hms(18, 11, 54)
        );
    }

    #[test]
    fn test_int_to_datetime_milliseconds() {
        assert_eq!(
            int_to_datetime(1630779114123).dt,
            Utc.ymd(2021, 9, 4).and_hms_milli(18, 11, 54, 123)
        );
    }

    #[test]
    fn test_int_to_datetime_microseconds() {
        assert_eq!(
            int_to_datetime(1630779114123456).dt,
            Utc.ymd(2021, 9, 4).and_hms_micro(18, 11, 54, 123456)
        );
    }

    #[test]
    fn test_int_to_datetime_nanoseconds() {
        assert_eq!(
            int_to_datetime(1630779114123456789).dt,
            Utc.ymd(2021, 9, 4).and_hms_nano(18, 11, 54, 123456789)
        );
    }
}
