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
    let mut t: i64 = 0;

    for r in input_data.trim().split(',') {
        if r.is_empty() {
            continue;
        }

        let parts: Vec<&str> = r.split('-').collect();
        let a: i64 = parts[0].parse().unwrap();
        let b: i64 = parts[1].parse().unwrap();

        for x in a..=b {
            let s = x.to_string();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2;
                if &s[..mid] == &s[mid..] {
                    t += x;
                }
            }
        }
    }

    t.to_string()
}
