// https://codeforces.com/problemset/problem/43/A
use std::collections::HashMap;

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
}

fn main() {
    let mut scores: HashMap<_, _> = HashMap::with_capacity(2);
    for _ in 0..input::scalar::<u8>() {
        *scores.entry(input::raw()).or_insert(0) += 1;
    }
    let key_with_max_value = scores.into_iter().max_by_key(|entry| entry.1).unwrap();
    print!("{}", key_with_max_value.0);
}
