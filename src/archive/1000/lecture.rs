// https://codeforces.com/problemset/problem/499/B
use std::collections::HashMap;

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
    let nm: Vec<u16> = input::vector(2);
    let mut dict: HashMap<_, _> = HashMap::with_capacity(nm[1] as usize);
    for _ in 0..nm[1] {
        let words: Vec<String> = input::vector(2);
        dict.insert(
            words[0].to_string(),
            if words[0].len() > words[1].len() {
                words[1].to_string()
            } else {
                words[0].to_string()
            },
        );
    }
    let words: Vec<String> = input::vector(nm[0] as usize);
    let sentence: Vec<String> = words
        .into_iter()
        .map(|w| dict.get(&w).unwrap().to_string())
        .collect();
    println!("{}", sentence.join(" "));
}
