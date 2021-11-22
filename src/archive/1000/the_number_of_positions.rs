// https://codeforces.com/problemset/problem/124/A
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

fn the_number_of_positions(n: u8, a: u8, b: u8) -> u8 {
    return u8::min(n-a, b+1);
}

fn main() {
    let nab: Vec<u8> = input::vector(3);
    println!("{}", the_number_of_positions(nab[0], nab[1], nab[2]));
}
