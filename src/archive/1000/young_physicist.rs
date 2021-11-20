// https://codeforces.com/problemset/problem/69/A
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
    let (mut x, mut y, mut z) = (0, 0, 0);
    for _ in 0..input::scalar::<u8>() {
        let xyz: Vec<i16> = input::vector(3);
        x += xyz[0];
        y += xyz[1];
        z += xyz[2];
    }
    match x == 0 && y == 0 && z == 0 {
        true => println!("YES"),
        false => println!("NO"),
    };
}
