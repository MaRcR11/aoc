use std::collections::HashMap;
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
    let parts: Vec<&str> = input_data.split("\n\n").collect();
    let mut s: HashMap<usize, usize> = HashMap::new();

    for x in &parts[..parts.len() - 1] {
        let l: Vec<&str> = x.lines().collect();
        let i: usize = l[0][..l[0].len() - 1].parse().unwrap();
        let size: usize = l[1..]
            .iter()
            .map(|r| r.chars().filter(|&c| c == '#').count())
            .sum();
        s.insert(i, size);
    }

    let mut t = 0;
    for r in parts[parts.len() - 1].lines() {
        if r.trim().is_empty() {
            continue;
        }
        let mut split = r.split(": ");
        let sz = split.next().unwrap();
        let ns: Vec<usize> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut rc = sz.split('x');
        let R: usize = rc.next().unwrap().parse().unwrap();
        let C: usize = rc.next().unwrap().parse().unwrap();
        let total: usize = ns
            .iter()
            .enumerate()
            .map(|(i, &n)| s.get(&i).unwrap_or(&0) * n)
            .sum();
        if (total as f64) * 1.3 < (R * C) as f64 {
            t += 1;
        }
    }
    t.to_string()
}
