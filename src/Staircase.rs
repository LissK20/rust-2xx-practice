use std::io::{self, BufRead};


fn staircase(n: i32)
{
    for i in 1..=n
    {
        println!("{:>width$}", "#".repeat(i as usize), width = n as usize);
    }
}

fn main()
{
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
