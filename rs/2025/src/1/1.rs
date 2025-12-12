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
    let mut p: i32 = 50;
    let mut t: i32 = 0;

    for line in input_data.lines() {
        if line.is_empty() {
            continue;
        }

        let (dir, num_str) = line.split_at(1);
        let value: i32 = num_str.parse().unwrap();

        let delta = if dir == "L" { -value } else { value };
        p = (p + delta).rem_euclid(100);

        if p == 0 {
            t += 1;
        }
    }

    t.to_string()
}
