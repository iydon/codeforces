// https://codeforces.com/problemset/problem/1343/A
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
    for _ in 0..input::scalar::<u16>() {
        let n: u32 = input::scalar();
        for k in 2..=31 {
            let x = u32::pow(2, k) - 1;
            if n % x == 0 {
                println!("{}", n / x);
                break;
            }
        }
    }
}
