use regex::Regex;
use std::fs::read_to_string;
use std::{env, format, i32};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn get_number_from(line: String) -> i32 {
    let li: String = line
        .clone()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let re = Regex::new(r"\d").unwrap();
    let matches: Vec<_> = re.find_iter(li.as_str()).map(|l| l.as_str()).collect();
    let n: usize = matches.len();
    let number: i32 = format!("{}{}", matches[0], matches[n - 1])
        .parse::<i32>()
        .unwrap();
    return number;
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
