// https://codeforces.com/problemset/problem/580/A
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
    let a: Vec<u32> = input::vector(input::scalar());
    let mut answer: u32 = 1;
    let mut number: u32 = 1;
    for group in a.windows(2) {
        if group[0] <= group[1] {
            number += 1;
        } else {
            if number > answer {
                answer = number;
            }
            number = 1;
        }
    }
    println!("{}", std::cmp::max(answer, number));
}
