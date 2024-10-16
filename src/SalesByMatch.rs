use std::collections::HashMap;
use std::io::{self, BufRead};

fn sockMerchant(_n: i32, ar: &[i32]) -> i32
{
    let mut color_count: HashMap<i32, i32> = HashMap::new();
    let mut pairs = 0;

    for &sock in ar
    {
        let count = color_count.entry(sock).or_insert(0);
        *count += 1;

        if *count == 2
        {
            pairs += 1;
            *count = 0;
        }
    }

    pairs
}

fn main()
{
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    println!("{}", result);
}
