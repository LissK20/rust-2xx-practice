use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String
{
    let (time, period) = s.split_at(s.len() - 2);

    let hours = &time[..2];
    let minutes_seconds = &time[2..];

    let mut hour: i32 = hours.parse().unwrap();

    if period == "AM"
    {
        if hour == 12
        {
            hour = 0;
        }
    }
    else if period == "PM"
    {
        if hour != 12
        {
            hour += 12;
        }
    }

    format!("{:02}{}", hour, minutes_seconds)
}

fn main()
{
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
