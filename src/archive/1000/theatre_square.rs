// https://codeforces.com/problemset/problem/1/A
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

fn theatre_square(n: u64, m: u64, a: u64) -> u64 {
    return ((n + a - 1) / a) * ((m + a - 1) / a);
}

fn main() {
    let nma: Vec<_> = input::vector(3);
    println!("{}", theatre_square(nma[0], nma[1], nma[2]));
}
