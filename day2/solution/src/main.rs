use regex::Regex;
use std::fs::read_to_string;
use std::{env, u32};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn parse_into_subsets(line: &str) -> (u32, Vec<String>) {
    let line_split: Vec<&str> = line.split(": ").collect();
    let id: u32 = line_split[0]
        .split("Game ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let _subsets: Vec<&str> = line_split[line_split.len() - 1].split("; ").collect();
    let mut subsets: Vec<String> = Vec::new();
    for _subset in _subsets {
        let mut s = _subset.clone().to_string();

        if !_subset.contains("red") {
            s = format!("{}, {}", s, "0 red");
        }

        if !_subset.contains("blue") {
            s = format!("{}, {}", s, "0 blue");
        }

        if !_subset.contains("green") {
            s = format!("{}, {}", s, "0 green");
        }

        subsets.push(s);
    }

    return (id, subsets);
}

fn game_possible(id: u32, subsets: Vec<String>) -> bool {
    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();
    let (rmax, gmax, bmax) = (12, 13, 14);
    for subset in subsets {
        let Some(r) = red.captures(subset.as_str()) else { return false };
        let Some(g) = green.captures(subset.as_str()) else { return false };
        let Some(b) = blue.captures(subset.as_str()) else { return false };
        let rr = &r[1].parse::<u32>().unwrap();
        let gg = &g[1].parse::<u32>().unwrap();
        let bb = &b[1].parse::<u32>().unwrap();

        if *rr > 12 {
            return false;
        }

        if *gg > 13 {
            return false;
        }

        if *bb > 14 {
            return false;
        }
    }
    return true;
}

fn powers_of(subsets: Vec<String>) -> u32 {
    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();
    let (mut rmin, mut gmin, mut bmin) = (0, 0, 0);
    for subset in subsets {
        let Some(r) = red.captures(subset.as_str()) else { return 0 };
        let Some(g) = green.captures(subset.as_str()) else { return 0 };
        let Some(b) = blue.captures(subset.as_str()) else { return 0 };
        let rr = &r[1].parse::<u32>().unwrap();
        let gg = &g[1].parse::<u32>().unwrap();
        let bb = &b[1].parse::<u32>().unwrap();

        if *rr > rmin {
            rmin = *rr;
        }

        if *gg > gmin {
            gmin = *gg;
        }

        if *bb > bmin {
            bmin = *bb;
        }
    }

    return rmin * gmin * bmin;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines: Vec<String> = read_lines(&args[1]);
    let mut possible: Vec<u32> = Vec::new();
    for line in &lines {
        let (id, subsets) = parse_into_subsets(&line);
        if game_possible(id, subsets) {
            possible.push(id);
        }
    }

    let sum: u32 = possible.iter().sum();
    println!("The sum of all possible game IDs in part 1 is: {}", sum);

    let mut powers: Vec<u32> = Vec::new();
    for line in &lines {
        let (_id, subsets) = parse_into_subsets(&line);
        powers.push(powers_of(subsets));
    }

    let sum_powers: u32 = powers.iter().sum();
    println!("The sum of all powers is: {}", sum_powers);
}
