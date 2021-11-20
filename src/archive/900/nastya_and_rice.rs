// https://codeforces.com/problemset/problem/1341/A
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

fn nastya_and_rice(n: u32, a: u32, b: u32, c: u32, d: u32) -> bool {
    let (left, right) = (n * (a - b), n * (a + b));
    return !(c + d < left || right < c - d);
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        let nabcd = input::vector(5);
        match nastya_and_rice(nabcd[0], nabcd[1], nabcd[2], nabcd[3], nabcd[4]) {
            true => println!("Yes"),
            false => println!("No"),
        };
    }
}
