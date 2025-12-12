use std::collections::VecDeque;
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
    let mut g: Vec<Vec<char>> = input_data
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let R = g.len();
    let C = g[0].len();
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut deg = vec![vec![0; C]; R];

    for r in 0..R {
        for c in 0..C {
            if g[r][c] == '@' {
                let mut count = 0;
                for &(dr, dc) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < R as isize && nc >= 0 && nc < C as isize {
                        if g[nr as usize][nc as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                deg[r][c] = count;
            }
        }
    }

    let mut q = VecDeque::new();
    for r in 0..R {
        for c in 0..C {
            if g[r][c] == '@' && deg[r][c] < 4 {
                q.push_back((r, c));
            }
        }
    }

    let mut t = 0;

    while let Some((r, c)) = q.pop_front() {
        if g[r][c] != '@' {
            continue;
        }
        g[r][c] = '.';
        t += 1;

        for &(dr, dc) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < R as isize && nc >= 0 && nc < C as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if g[nr][nc] == '@' {
                    deg[nr][nc] -= 1;
                    if deg[nr][nc] == 3 {
                        q.push_back((nr, nc));
                    }
                }
            }
        }
    }

    t.to_string()
}
