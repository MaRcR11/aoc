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
    let lines: Vec<&str> = input_data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    let g: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let r_len = g.len();
    let c_len = if g.is_empty() { 0 } else { g[0].len() };

    let mut s = None;
    for r in 0..r_len {
        for c in 0..c_len {
            if g[r][c] == 'S' {
                s = Some((r, c));
                break;
            }
        }
        if s.is_some() {
            break;
        }
    }

    let mut memo: HashMap<Option<(usize, usize)>, i64> = HashMap::new();

    fn cnt(
        n: Option<(usize, usize)>,
        g: &Vec<Vec<char>>,
        r_len: usize,
        c_len: usize,
        memo: &mut HashMap<Option<(usize, usize)>, i64>,
    ) -> i64 {
        if let Some(&cached) = memo.get(&n) {
            return cached;
        }

        if n.is_none() {
            return 1;
        }

        let (r, c) = n.unwrap();
        let ch = g[r][c];
        let mut neighbors = Vec::new();

        if ch == '^' {
            for nc in [c.wrapping_sub(1), c + 1] {
                if nc < c_len {
                    neighbors.push(Some((r, nc)));
                } else {
                    neighbors.push(None);
                }
            }
        } else {
            let nr = r + 1;
            if nr < r_len {
                neighbors.push(Some((nr, c)));
            } else {
                neighbors.push(None);
            }
        }

        let result = neighbors
            .iter()
            .map(|&x| cnt(x, g, r_len, c_len, memo))
            .sum();
        memo.insert(n, result);
        result
    }

    let t = cnt(s, &g, r_len, c_len, &mut memo);
    t.to_string()
}
