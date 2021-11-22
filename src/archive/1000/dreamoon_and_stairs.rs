// https://codeforces.com/problemset/problem/476/A
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

fn dreamoon_and_stairs(n: u16, m: u16) -> Option<u16> {
    for x in ((n+1)/2)..=n {
        if x%m == 0 {
            return Some(x);
        }
    }
    return None;
}

fn main() {
    let nm: Vec<u16> = input::vector(2);
    match dreamoon_and_stairs(nm[0], nm[1]) {
        Some(n) => println!("{}", n),
        None => println!("-1"),
    };
}
