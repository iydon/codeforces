// https://codeforces.com/problemset/problem/1335/B
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

fn chr(i: u8) -> char {
    return (97 + i) as char;
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        let nab: Vec<u16> = input::vector(3);
        for n in 0..nab[0] {
            print!("{}", chr((n % nab[2]) as u8));
        }
        println!();
    }
}
