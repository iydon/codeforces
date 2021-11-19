// https://codeforces.com/problemset/problem/460/A
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

fn vasya_and_socks(n: u32, m: u32) -> u32 {
    return (n*m-1) / (m-1);
}

fn main() {
    let nm: Vec<_> = input::vector(2);
    println!("{}", vasya_and_socks(nm[0], nm[1]));
}
