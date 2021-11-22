// https://codeforces.com/problemset/problem/1342/A
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

fn road_to_zero(x: u64, y: u64, a: u64, b: u64) -> u64 {
    let total = x + y;
    let min = u64::min(x, y);
    // plan a
    let plan_a = a * (x + y);
    // plan b
    let plan_b = b * min + a * (total - 2 * min);

    return u64::min(plan_a, plan_b);
}

fn main() {
    for _ in 0..input::scalar::<u8>() {
        let xy: Vec<u64> = input::vector(2);
        let ab: Vec<u64> = input::vector(2);
        println!("{}", road_to_zero(xy[0], xy[1], ab[0], ab[1]));
    }
}
