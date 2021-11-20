// https://codeforces.com/problemset/problem/339/B
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
    let nm: Vec<u64> = input::vector(2);
    let a: Vec<u64> = input::vector(nm[1] as usize);
    let sum = a
        .iter()
        .skip(1)
        .zip(a.iter())
        .filter(|(x, y)| x < y)
        .count();
    println!("{}", nm[0] * sum as u64 + a.last().unwrap() - 1);
}
