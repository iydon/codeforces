// https://codeforces.com/problemset/problem/1475/A
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

fn odd_divisor(mut n: u64) -> bool {
    while n%2 == 0 {
        n /= 2;
    }
    return n%2==1 && n>2;
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        println!("{}", if odd_divisor(input::scalar()) {"YES"} else {"NO"})
    }
}
