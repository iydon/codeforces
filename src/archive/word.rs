// https://codeforces.com/problemset/problem/59/A
use std::convert::TryInto;
use std::io::Write;
use std::str::FromStr;

#[allow(non_camel_case_types)]
struct input;

#[allow(dead_code)]
impl input {
    pub fn prompt(text: &str) -> () {
        print!("{}", text);
        std::io::stdout().flush().unwrap();
    }

    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }

    pub fn text() -> String {
        return Self::raw().trim().to_string();
    }

    pub fn scalar<T>() -> T where
        T: FromStr {
        return Self::raw().trim()
            .parse()
            .unwrap_or_else(|_| panic!());
    }

    pub fn vector<T>(length: usize) -> Vec<T> where
        T: FromStr {
        return Self::raw().trim()
            .split_whitespace().take(length)
            .map(|s| s.parse().unwrap_or_else(|_| panic!()))
            .collect();
    }

    pub fn array<T, const N: usize>() -> [T; N] where
        T: FromStr {
        // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
        return Self::vector(N)
            .try_into()
            .unwrap_or_else(|_| panic!());
    }
}

fn main() {
    let word = input::text();
    let mut delta: i8 = 0;
    for c in word.chars() {
        delta += match c.is_ascii_lowercase() {
            true => 1,
            false => -1,
        };
    }
    println!("{}", if delta<0 {word.to_ascii_uppercase()} else {word.to_ascii_lowercase()});
}
