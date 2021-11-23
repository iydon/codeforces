// https://codeforces.com/problemset/problem/706/B
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
    let n: u32 = input::scalar();
    let mut map = [0u32; 100_000];
    for x in input::vector::<u32>(n as usize) {
        map[x as usize - 1] += 1;
    }
    for ith in 1..100_000 {
        map[ith] += map[ith - 1];
    }
    for _ in 0..input::scalar::<u32>() {
        let m: u32 = input::scalar();
        if m >= 100_000 {
            println!("{}", n);
        } else {
            println!("{}", map[m as usize - 1]);
        }
    }
}
