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
    let parts: Vec<&str> = input_data.trim().split("\n\n").collect();
    let mut ranges: Vec<(i64, i64)> = parts[0]
        .lines()
        .map(|l| {
            let nums: Vec<i64> = l.split('-').map(|x| x.parse().unwrap()).collect();
            (nums[0], nums[1])
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

    let ids: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    fn in_ranges(x: i64, merged: &[[i64; 2]]) -> bool {
        let mut left = 0;
        let mut right = merged.len() as isize - 1;
        while left <= right {
            let mid = (left + right) / 2;
            let mid_range = merged[mid as usize];
            if mid_range[0] <= x && x <= mid_range[1] {
                return true;
            } else if x < mid_range[0] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }

    let t = ids.iter().filter(|&&x| in_ranges(x, &merged)).count();

    t.to_string()
}
