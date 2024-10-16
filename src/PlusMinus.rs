use std::io::{self, BufRead};

fn plusMinus(arr: &[i32])
{
    let total = arr.len() as f64;
    let mut positives = 0;
    let mut negatives = 0;
    let mut zeros = 0;

    for &num in arr
    {
        if num > 0
        {
            positives += 1;
        }
        else if num < 0
        {
            negatives += 1;
        }
        else
        {
            zeros += 1;
        }
    }

    println!("{:.6}", positives as f64 / total);
    println!("{:.6}", negatives as f64 / total);
    println!("{:.6}", zeros as f64 / total);
}

fn main()
{
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
