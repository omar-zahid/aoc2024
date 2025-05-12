use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input-3").expect("No file.");
    let input =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

    let mul_match = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;

    let new_input = process_input(&input);
    println!("{new_input}");

    for line in new_input.lines() {
        let caps: usize = mul_match
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

    println!("Result for Day 3, part 2 is {}", result);
}

fn process_input(input: &str) -> String {
    let pat = Regex::new(r"(.*)don't\(\)(.*)do\(\)").unwrap();

    let line: String = input.split_inclusive("don't()").collect();

    let out: String = pat
        .captures_iter(&line)
        .map(|caps| {
            let (_, [a, _b]) = caps.extract();
            a
        })
        .collect();

    out
}
