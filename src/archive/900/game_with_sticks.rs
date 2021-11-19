// https://codeforces.com/problemset/problem/451/A
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
    let nm: Vec<u8> = input::vector(2);
    let min = std::cmp::min(nm[0], nm[1]);
    if min % 2 == 0 {
        println!("Malvika");
    } else {
        println!("Akshat");
    }
}
