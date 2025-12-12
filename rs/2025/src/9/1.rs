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
    let pts: Vec<(i64, i64)> = input_data
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let nums: Vec<i64> = l.split(',').map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let mut t = 0i64;
    let n = pts.len();

    for i in 0..n {
        let (x1, y1) = pts[i];
        for j in (i + 1)..n {
            let (x2, y2) = pts[j];
            let val = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
            t = t.max(val);
        }
    }

    t.to_string()
}
