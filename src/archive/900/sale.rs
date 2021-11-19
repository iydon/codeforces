// https://codeforces.com/problemset/problem/34/B
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
    let nm: Vec<_> = input::vector(2);
    let mut a: Vec<i32> = input::vector(nm[0])
        .into_iter()
        .filter(|&a| a<0)
        .collect();
    a.sort();
    let ans: i32 = a.iter().take(nm[1]).sum();
    println!("{}", -ans);
}
