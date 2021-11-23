// https://codeforces.com/problemset/problem/456/A
use std::collections::HashMap;

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
    let mut map = HashMap::with_capacity(n);
    for _ in 0..n {
        let ab: Vec<u32> = input::vector(2);
        map.insert(ab[1], ab[0]);
    }
    let mut map: Vec<(u32, u32)> = map.into_iter().collect();
    map.sort_by(|x, y| x.0.cmp(&y.0));
    let map = map.into_iter().map(|x| x.1).collect::<Vec<_>>();
    match map.iter().skip(1).zip(map.iter()).all(|(x, y)| x>y) {
        true  => println!("Poor Alex"),
        false => println!("Happy Alex"),
    };
}
