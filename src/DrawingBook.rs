use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pageCount(n: i32, p: i32) -> i32
{
    let from_front = p / 2;

    let from_back = if n % 2 == 0
    {
        (n - p + 1) / 2
    }
    else
    {
        (n - p) / 2
    };

    from_front.min(from_back)
}

fn main()
{
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    writeln!(&mut fptr, "{}", result).ok();
}
