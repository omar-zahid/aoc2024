use std::fs;

fn main() {
    let input = fs::read_to_string("input-2").expect("No file.");

    let mut reports = 0;

    for line in input.lines() {
        let levels: Vec<usize> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        if is_safe(&levels) {
            reports += 1;
        } else {
            let mut safe_with_removal = false;
            for i in 0..levels.len() {
                let mut temp = levels.clone();
                temp.remove(i);
                if is_safe(&temp) {
                    safe_with_removal = true;
                    break;
                }
            }

            if safe_with_removal {
                reports += 1;
            }
        }
    }

    println!("Number of safe reports::: {}", reports);
}

fn is_safe(levels: &[usize]) -> bool {
    let mut direction: Option<&str> = None;

    for window in levels.windows(2) {
        let a = window[0];
        let b = window[1];

        let diff = a.abs_diff(b);

        if diff > 3 || diff < 1 {
            return false;
        }

        let current_dir = if b > a {
            Some("inc")
        } else if b < a {
            Some("dec")
        } else {
            return false;
        };

        if let Some(dir) = direction {
            if dir != current_dir.unwrap() {
                return false;
            }
        } else {
            direction = current_dir;
        }
    }
    return true;
}
