use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn read_input_vec(filename: &str) -> Result<Vec<i64>, std::io::Error> {
    Ok(io::BufReader::new(File::open(filename)?)
        .lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<i64>()
                .expect("Could not parse number")
        })
        .collect())
}

fn two_sum(input: &[i64], target: i64) -> Option<i64> {
    let mut found: HashSet<i64> = HashSet::with_capacity(input.len());
    for num in input {
        let complement = target - num;
        if found.contains(&complement) {
            return Some(*num);
        }
        found.insert(*num);
    }
    None
}

const TARGET: i64 = 2020;
fn main() -> Result<(), std::io::Error> {
    let input = read_input_vec("day1/input.txt")?;

    match two_sum(&input, TARGET) {
        Some(num) => println!(
            "{} + {} = {}\nAnswer: {}",
            num,
            (TARGET - num),
            TARGET,
            num * (TARGET - num)
        ),
        None => println!("Did not find solution :("),
    };

    let mut threesum_vec: Vec<i64> = Vec::with_capacity(input.len());
    for num in &input {
        let threesum_complement = TARGET - num;
        if threesum_complement > 0 {
            threesum_vec.push(threesum_complement);
        }
    }

    for complement_target in threesum_vec {
        if let Some(two_sum_answer) = two_sum(&input, complement_target) {
            let num1 = two_sum_answer;
            let num2 = complement_target - two_sum_answer;
            let num3 = TARGET - complement_target;
            println!("{} + {} + {} = {}", num1, num2, num3, TARGET);
            assert!(num1 + num2 + num3 == TARGET);
            println!("Answer: {}", num1 * num2 * num3);
            return Ok(());
        }
    }

    println!("Did not find solution :(");
    Ok(())
}
