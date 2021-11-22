// https://codeforces.com/problemset/problem/500/A
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
    let nt: Vec<u16> = input::vector(2);
    let a: Vec<u16> = input::vector(nt[0] as usize - 1);
    let mut is_accessed = vec![false; nt[0] as usize];
    is_accessed[0] = true;
    for ith in 0..is_accessed.len() - 1 {
        if is_accessed[ith] {
            is_accessed[ith + a[ith] as usize] = true;
        }
    }
    if is_accessed[nt[1] as usize - 1] {
        println!("YES");
    } else {
        println!("NO");
    }
}
