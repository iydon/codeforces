// https://codeforces.com/problemset/problem/379/A
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

fn new_year_candles(mut a: u16, b: u16) -> u16 {
    let mut ans = 0;
    let mut r = 0;
    while a != 0 {
        let total = a + r;
        ans += a;
        a = total / b;
        r = total % b;
    }
    return ans;
}

fn main() {
    let ab: Vec<u16> = input::vector(2);
    println!("{}", new_year_candles(ab[0], ab[1]));
}
