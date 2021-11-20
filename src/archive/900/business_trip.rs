// https://codeforces.com/problemset/problem/149/A
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

fn business_trip(a: Vec<u8>, k: u8) -> Option<u8> {
    if k == 0 {
        return Some(0);
    }
    let mut sum: u8 = 0;
    for (ith, a) in a.into_iter().enumerate() {
        sum += a;
        if sum >= k {
            return Some((ith + 1) as u8);
        }
    }
    return None;
}

fn main() {
    let k: u8 = input::scalar();
    let mut a: Vec<u8> = input::vector(12);
    a.sort_by(|x, y| y.cmp(x));
    match business_trip(a, k) {
        Some(k) => println!("{}", k),
        None => println!("-1"),
    };
}
