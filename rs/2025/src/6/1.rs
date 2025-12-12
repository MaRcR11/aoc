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
    let lines: Vec<&str> = input_data.trim_end_matches('\n').lines().collect();
    let h = lines.len();
    let w = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let g: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut v: Vec<char> = l.chars().collect();
            v.resize(w, ' ');
            v
        })
        .collect();

    let sc: Vec<bool> = (0..w).map(|c| (0..h).all(|r| g[r][c] == ' ')).collect();

    let mut seg: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < w {
        if sc[i] {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < w && !sc[j] {
            j += 1;
        }
        seg.push((i, j));
        i = j;
    }

    let mut t: i128 = 0;
    for (a, b) in seg {
        let mut nums: Vec<i128> = Vec::new();
        for r in 0..(h - 1) {
            let s: String = g[r][a..b].iter().collect::<String>().trim().to_string();
            if !s.is_empty() {
                if let Ok(val) = s.parse::<i128>() {
                    nums.push(val);
                }
            }
        }

        let op: String = g[h - 1][a..b].iter().collect::<String>().trim().to_string();
        let mut v: i128 = if op == "+" { nums.iter().sum() } else { 1 };
        if op != "+" {
            for &x in &nums {
                v *= x;
            }
        }
        t += v;
    }

    t.to_string()
}
