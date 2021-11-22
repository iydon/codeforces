// https://codeforces.com/problemset/problem/577/A
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

fn multiplication_table(n: u64, x: u64) -> u64 {
    let mut ans = 0;
    for ith in 1..=n {
        if x%ith==0 && x<=ith*n {
            ans += 1;
        }
    }
    return ans;
}

fn main() {
    let nx: Vec<u64> = input::vector(2);
    println!("{}", multiplication_table(nx[0], nx[1]));
}
