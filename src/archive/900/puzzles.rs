// https://codeforces.com/problemset/problem/337/A
mod input {
    use std::str::FromStr;

    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }

    pub fn vector<T>(length: usize) -> Vec<T>
    where
        T: FromStr,
    {
        return raw()
            .trim()
            .split_whitespace()
            .take(length)
            .map(|s| s.parse().unwrap_or_else(|_| panic!()))
            .collect();
    }
}

fn main() {
    let nm: Vec<usize> = input::vector(2);
    let mut f: Vec<u16> = input::vector(nm[1]);
    let mut answer: u16 = 999;
    f.sort();
    for w in f.windows(nm[0]) {
        let delta = w.last().unwrap() - w[0];
        if delta < answer {
            answer = delta;
        }
    }
    println!("{}", answer);
}
