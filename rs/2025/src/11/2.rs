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
    let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input_data.lines() {
        if let Some((a, b)) = line.split_once(':') {
            g.insert(a.trim(), b.split_whitespace().collect());
        }
    }

    let st = "svr";
    let tgt = "out";
    let nd: HashMap<&str, u8> = [("dac", 1), ("fft", 2)].iter().cloned().collect();

    use std::collections::HashMap as Memo;

    fn dfs<'a>(
        x: &'a str,
        mask: u8,
        tgt: &str,
        g: &HashMap<&'a str, Vec<&'a str>>,
        nd: &HashMap<&'a str, u8>,
        memo: &mut Memo<(&'a str, u8), u64>,
    ) -> u64 {
        if let Some(&res) = memo.get(&(x, mask)) {
            return res;
        }

        if x == tgt {
            let res = if mask == 3 { 1 } else { 0 };
            memo.insert((x, mask), res);
            return res;
        }

        let mut m = mask;
        if let Some(&bit) = nd.get(x) {
            m |= bit;
        }

        let mut t: u64 = 0;
        if let Some(neighbors) = g.get(x) {
            for &y in neighbors {
                t += dfs(y, m, tgt, g, nd, memo);
            }
        }

        memo.insert((x, mask), t);
        t
    }

    let mut memo: HashMap<(&str, u8), u64> = HashMap::new();
    let t = dfs(st, 0, tgt, &g, &nd, &mut memo);
    t.to_string()
}
