// https://codeforces.com/problemset/problem/455/A
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
    const LENGTH: usize = 100000;
    let a: Vec<i64> = input::vector(input::scalar());
    let mut points: [i64; LENGTH + 2] = [0; LENGTH + 2];
    for ith in a {
        points[ith as usize] += ith;
    }
    for ith in (0..LENGTH).rev() {
        points[ith] = std::cmp::max(points[ith + 1], points[ith + 2] + points[ith]);
    }
    println!("{}", points[0]);
}
