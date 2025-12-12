use regex::Regex;
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
    let lines: Vec<&str> = input_data.trim().lines().collect();
    let mut t = 0;

    let bracket_re = Regex::new(r"\[(.*?)\]").unwrap();
    let paren_re = Regex::new(r"\((.*?)\)").unwrap();

    for l in lines {
        let pat = bracket_re.captures(l).unwrap().get(1).unwrap().as_str();
        let tgt: Vec<i32> = pat.chars().map(|c| if c == '#' { 1 } else { 0 }).collect();
        let n = tgt.len();

        let mut btns: Vec<Vec<usize>> = Vec::new();
        for cap in paren_re.captures_iter(l) {
            let b = cap.get(1).unwrap().as_str();
            if !b.trim().is_empty() {
                let nums: Vec<usize> = b.split(',').map(|s| s.trim().parse().unwrap()).collect();
                btns.push(nums);
            }
        }

        let m = btns.len();
        let mut a = vec![vec![0i32; m]; n];

        for (j, lst) in btns.iter().enumerate() {
            for &i in lst {
                a[i][j] = 1;
            }
        }

        let mut best = i32::MAX;

        for mask in 0..(1 << m) {
            let mut vec = vec![0i32; n];
            let mut p = 0;

            for j in 0..m {
                if (mask >> j) & 1 == 1 {
                    p += 1;
                    for i in 0..n {
                        if a[i][j] == 1 {
                            vec[i] ^= 1;
                        }
                    }
                }
            }

            if vec == tgt && p < best {
                best = p;
            }
        }

        t += best;
    }

    t.to_string()
}
