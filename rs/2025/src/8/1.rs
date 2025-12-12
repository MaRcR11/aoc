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
    let pts: Vec<(i64, i64, i64)> = input_data
        .trim()
        .lines()
        .map(|l| {
            let nums: Vec<i64> = l.split(',').map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();

    let n = pts.len();
    let mut d: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..n {
        let (x1, y1, z1) = pts[i];
        for j in (i + 1)..n {
            let (x2, y2, z2) = pts[j];
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;
            d.push((dx * dx + dy * dy + dz * dz, i, j));
        }
    }

    d.sort_unstable();

    let mut p: Vec<usize> = (0..n).collect();
    let mut sz: Vec<usize> = vec![1; n];

    fn find(x: usize, p: &mut Vec<usize>) -> usize {
        let mut curr = x;
        while p[curr] != curr {
            p[curr] = p[p[curr]];
            curr = p[curr];
        }
        curr
    }

    fn union(a: usize, b: usize, p: &mut Vec<usize>, sz: &mut Vec<usize>) {
        let mut ra = find(a, p);
        let mut rb = find(b, p);
        if ra == rb {
            return;
        }
        if sz[ra] < sz[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        p[rb] = ra;
        sz[ra] += sz[rb];
    }

    for i in 0..1000.min(d.len()) {
        let (_, a, b) = d[i];
        union(a, b, &mut p, &mut sz);
    }

    let mut c: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let r = find(i, &mut p);
        *c.entry(r).or_insert(0) += 1;
    }

    let mut s: Vec<usize> = c.values().cloned().collect();
    s.sort_unstable_by(|a, b| b.cmp(a));

    let t = s[0] * s[1] * s[2];
    t.to_string()
}
