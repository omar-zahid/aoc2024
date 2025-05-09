use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").expect("No file.");

    let mut l: Vec<usize> = Vec::new();
    let mut r = HashMap::new();

    for line in input.lines() {
        let item: Vec<usize> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        l.push(item[0]);

        let count = r.entry(item[1]).or_insert(0);
        *count += 1;
    }

    let sum: usize = l
        .iter()
        .map(|x| {
            let count = r.get(x);
            match count {
                Some(c) => x * c,
                None => 0,
            }
        })
        .sum();

    println!("{sum:?}");
}
