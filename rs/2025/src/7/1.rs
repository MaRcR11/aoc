use std::collections::HashSet;
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
    let g: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let r_len = g.len();
    let c_len = if g.is_empty() { 0 } else { g[0].len() };

    let mut sr = 0;
    let mut sc = 0;
    let mut found = false;
    for r in 0..r_len {
        for c in 0..c_len {
            if g[r][c] == 'S' {
                sr = r;
                sc = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let mut t = 0;
    let mut b: Vec<(usize, usize, String)> = vec![(sr, sc, "down".to_string())];
    let mut v: HashSet<(usize, usize)> = HashSet::new();

    while !b.is_empty() {
        let mut nb: Vec<(usize, usize, String)> = Vec::new();
        for (row, col, d) in b {
            if d == "down" {
                let mut r = row + 1;
                while r < r_len {
                    if g[r][col] == '^' {
                        let s = (r, col);
                        if !v.contains(&s) {
                            v.insert(s);
                            t += 1;
                            if col > 0 {
                                nb.push((r, col - 1, "left".to_string()));
                            }
                            if col + 1 < c_len {
                                nb.push((r, col + 1, "right".to_string()));
                            }
                        }
                        break;
                    }
                    r += 1;
                }
            } else if d == "left" {
                let mut c = col;
                loop {
                    if c < c_len && g[row][c] == '^' {
                        let s = (row, c);
                        if !v.contains(&s) {
                            v.insert(s);
                            t += 1;
                            if c > 0 {
                                nb.push((row, c - 1, "left".to_string()));
                            }
                            if c + 1 < c_len {
                                nb.push((row, c + 1, "right".to_string()));
                            }
                        }
                        break;
                    }
                    if row + 1 < r_len {
                        nb.push((row + 1, c, "down".to_string()));
                    }
                    break;
                }
            } else if d == "right" {
                let mut c = col;
                loop {
                    if c < c_len && g[row][c] == '^' {
                        let s = (row, c);
                        if !v.contains(&s) {
                            v.insert(s);
                            t += 1;
                            if c > 0 {
                                nb.push((row, c - 1, "left".to_string()));
                            }
                            if c + 1 < c_len {
                                nb.push((row, c + 1, "right".to_string()));
                            }
                        }
                        break;
                    }
                    if row + 1 < r_len {
                        nb.push((row + 1, c, "down".to_string()));
                    }
                    break;
                }
            }
        }
        b = nb;
    }

    t.to_string()
}
