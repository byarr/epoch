use chrono::{DateTime, Local, Utc};
use epoch::try_parse;
use std::env::args_os;
use std::process::exit;

fn main() {
    let args: Vec<_> = args_os().collect();

    if args.len() == 1 {
        let date_time = Utc::now();

        print_date_time(&date_time);
        return;
    }

    if args.len() != 2 {
        eprintln!("Expected 1 argument got {}", args.len() - 1);
        exit(1);
    }

    let input = std::env::args().nth(1).expect("No time given");
    let option = try_parse(&input);

    match option {
        None => {
            println!("Failed to parse argument")
        }
        Some(dt) => {
            println!("Assuming {}", dt.unit);
            print_date_time(&dt.dt);
        }
    }
}

fn print_date_time(date_time: &DateTime<Utc>) {
    let as_local = date_time.with_timezone(&Local);

    println!("UTC:        {}", date_time.to_rfc3339());
    println!("Local:      {}", as_local.to_rfc3339());

    println!("Epoch (s):  {}", date_time.timestamp());
    println!("Epoch (ms): {}", date_time.timestamp_millis());
}
