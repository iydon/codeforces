// https://codeforces.com/problemset/problem/318/A
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
    let ns: Vec<u64> = input::vector(2);
    let n = ns[0];
    let k = ns[1];
    let mid = (n + 1) / 2;
    let answer = if k <= mid { 2 * k - 1 } else { 2 * (k - mid) };
    println!("{}", answer);
}
