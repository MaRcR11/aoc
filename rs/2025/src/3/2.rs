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

    for line in input_data.lines() {
        let digits: Vec<i32> = line
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect();

        let n = digits.len();
        let mut s: Vec<i32> = Vec::new();
        let mut r = n as i32 - 12;

        for &x in &digits {
            while !s.is_empty() && r > 0 && *s.last().unwrap() < x {
                s.pop();
                r -= 1;
            }
            s.push(x);
        }

        let num_str: String = s.iter().take(12).map(|d| d.to_string()).collect();
        t += num_str.parse::<i64>().unwrap();
    }

    t.to_string()
}
