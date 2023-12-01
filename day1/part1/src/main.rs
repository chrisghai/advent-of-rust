use regex::Regex;
use std::{env, i32, format};
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn get_number_from(line: String) -> i32 {
    let re = Regex::new(r"\d").unwrap();
    let matches: Vec<_> = re.find_iter(line.as_str()).map(|l| l.as_str()).collect();
    let n: usize = matches.len();
    return format!("{}{}", matches[0], matches[n - 1]).parse::<i32>().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines: Vec<String> = read_lines(&args[1]);
    let mut numbers: Vec<i32> = vec![];
    for line in lines {
        numbers.push(get_number_from(line));
    }

    let sum: i32 = numbers.iter().sum();
    println!("The sum of all calibration numbers is: {}", sum);
}
