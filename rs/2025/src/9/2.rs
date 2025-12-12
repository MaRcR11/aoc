use std::collections::{HashMap, HashSet, VecDeque};
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

    let n = pts.len();

    let mut xs_set: HashSet<i64> = pts.iter().map(|&(x, _)| x).collect();
    let mut ys_set: HashSet<i64> = pts.iter().map(|&(_, y)| y).collect();

    let mut xs: Vec<i64> = xs_set.into_iter().collect();
    let mut ys: Vec<i64> = ys_set.into_iter().collect();
    xs.sort_unstable();
    ys.sort_unstable();

    let xv: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let yv: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let w = xs.len();
    let h = ys.len();

    let mut g = vec![vec![0i32; h]; w];

    for i in 0..n {
        let (x1, y1) = pts[i];
        let (x2, y2) = pts[(i + 1) % n];

        if x1 == x2 {
            let x = xv[&x1];
            let mut a = yv[&y1];
            let mut b = yv[&y2];
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            for y in a..=b {
                g[x][y] = 1;
            }
        } else {
            let y = yv[&y1];
            let mut a = xv[&x1];
            let mut b = xv[&x2];
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            for x in a..=b {
                g[x][y] = 1;
            }
        }
    }

    let mut q = VecDeque::new();

    for x in 0..w {
        for &y in &[0, h - 1] {
            if g[x][y] == 0 {
                g[x][y] = -1;
                q.push_back((x, y));
            }
        }
    }

    for y in 0..h {
        for &x in &[0, w - 1] {
            if g[x][y] == 0 {
                g[x][y] = -1;
                q.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = q.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if g[nx][ny] == 0 {
                    g[nx][ny] = -1;
                    q.push_back((nx, ny));
                }
            }
        }
    }

    for x in 0..w {
        for y in 0..h {
            g[x][y] = if g[x][y] != -1 { 1 } else { 0 };
        }
    }

    let mut ps = vec![vec![0i64; h + 1]; w + 1];
    for x in 0..w {
        let mut s = 0i64;
        for y in 0..h {
            s += g[x][y] as i64;
            ps[x + 1][y + 1] = ps[x][y + 1] + s;
        }
    }

    let ok = |x1: usize, y1: usize, x2: usize, y2: usize| -> bool {
        let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        let tot = ((x2 - x1 + 1) * (y2 - y1 + 1)) as i64;
        let s = ps[x2 + 1][y2 + 1] - ps[x1][y2 + 1] - ps[x2 + 1][y1] + ps[x1][y1];
        s == tot
    };

    let mut t = 0i64;
    for i in 0..n {
        let (x1, y1) = pts[i];
        for j in (i + 1)..n {
            let (x2, y2) = pts[j];
            if ok(xv[&x1], yv[&y1], xv[&x2], yv[&y2]) {
                t = t.max(((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1));
            }
        }
    }

    t.to_string()
}
