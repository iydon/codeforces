// https://codeforces.com/problemset/problem/519/B
mod input {
    use std::str::FromStr;

    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }

    pub fn scalar<T>() -> T
    where
        T: FromStr,
    {
        return raw().trim().parse().unwrap_or_else(|_| panic!());
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
    let n: usize = input::scalar();
    let a: u64 = input::vector(n).iter().sum();
    let b: u64 = input::vector(n - 1).iter().sum();
    let c: u64 = input::vector(n - 2).iter().sum();
    println!("{}", a-b);
    println!("{}", b-c);
}
