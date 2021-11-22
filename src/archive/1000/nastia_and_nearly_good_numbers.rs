// https://codeforces.com/problemset/problem/1521/A
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

fn nastia_and_nearly_good_numbers(a: u64, b: u64) -> Option<[u64; 3]> {
    if b == 1 {
        return None;
    } else {
        return Some([a, a*b, a*(b+1)]);
    }
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        let ab: Vec<u64> = input::vector(2);
        match nastia_and_nearly_good_numbers(ab[0], ab[1]) {
            Some([x, y, z]) => println!("YES\n{} {} {}", x, y, z),
            None => println!("NO"),
        };
    }
}
