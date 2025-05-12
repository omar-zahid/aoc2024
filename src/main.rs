use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input-3").expect("No file.");
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;

    for line in input.lines() {
        let caps: usize = regex
            .captures_iter(line)
            .map(|caps| {
                let (_, [a, b]) = caps.extract();
                let left = a.parse::<usize>().unwrap_or(0);
                let right = b.parse::<usize>().unwrap_or(0);
                left * right
            })
            .sum();

        result = result + caps;
    }

    println!("Result for Day 3, part 1 is {}", result);
}
