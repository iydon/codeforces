// https://codeforces.com/problemset/problem/368/B
use std::collections::HashSet;

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
    let nm: Vec<_> = input::vector(2);
    let mut set: HashSet<_> = HashSet::with_capacity(nm[0]);
    let a: Vec<u32> = input::vector(nm[0])
        .into_iter()
        .rev()
        .map(|a: u32| {
            set.insert(a);
            set.len() as u32
        })
        .collect();
    for _ in 0..nm[1] {
        let l: usize = input::scalar();
        println!("{}", a[a.len()-l]);
        writeln!(io::std)
    }
}
