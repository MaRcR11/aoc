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
        let digits: Vec<i64> = line
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i64)
            .collect();

        let mut max_val = 0;
        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let val = digits[i] * 10 + digits[j];
                if val > max_val {
                    max_val = val;
                }
            }
        }

        t += max_val;
    }

    t.to_string()
}
