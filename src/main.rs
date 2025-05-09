use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("No file.");

    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();

    for line in input.lines() {
        let item: Vec<usize> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        l.push(item[0]);
        r.push(item[1]);
    }

    l.sort();
    r.sort();

    let line = l.into_iter().zip(r);
    let sum: usize = line.map(|x| x.0.abs_diff(x.1)).sum();

    println!("{sum}");
}
