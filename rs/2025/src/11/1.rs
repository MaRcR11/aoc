use std::collections::{HashMap, HashSet};
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
    let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input_data.lines() {
        if let Some((a, b)) = line.split_once(':') {
            let neighbors: Vec<&str> = b.split_whitespace().collect();
            g.insert(a.trim(), neighbors);
        }
    }

    let tgt = "out";
    let st = "you";

    fn dfs<'a>(
        x: &'a str,
        tgt: &str,
        g: &HashMap<&'a str, Vec<&'a str>>,
        seen: &mut HashSet<&'a str>,
    ) -> u32 {
        if x == tgt {
            return 1;
        }
        let mut t = 0;
        if let Some(neighbors) = g.get(x) {
            for &y in neighbors {
                if !seen.contains(y) {
                    seen.insert(y);
                    t += dfs(y, tgt, g, seen);
                    seen.remove(y);
                }
            }
        }
        t
    }

    let mut seen = HashSet::new();
    seen.insert(st);
    let t = dfs(st, tgt, &g, &mut seen);

    t.to_string()
}
