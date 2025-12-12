use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = solve(&input.trim());
    println!("{}", result);
}

fn solve(input_data: &str) -> String {
    let mut ranges: Vec<(i64, i64)> = input_data
        .trim()
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                return None;
            }
            l.split_once('-')
                .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        })
        .collect();

    ranges.sort_by_key(|&(a, _)| a);

    let mut merged: Vec<[i64; 2]> = Vec::new();
    for (a, b) in ranges {
        if merged.is_empty() || a > merged.last().unwrap()[1] + 1 {
            merged.push([a, b]);
        } else {
            merged.last_mut().unwrap()[1] = merged.last().unwrap()[1].max(b);
        }
    }

    let t: i64 = merged.iter().map(|[a, b]| b - a + 1).sum();

    t.to_string()
}
