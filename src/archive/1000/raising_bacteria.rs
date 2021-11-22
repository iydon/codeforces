// https://codeforces.com/problemset/problem/579/A
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

fn raising_bacteria(mut x: u32) -> u32 {
    let mut ans = 0;
    while x != 0 {
        ans += x % 2;
        x /= 2;
    }
    return ans;
}

fn main() {
    let x: u32 = input::scalar();
    println!("{}", raising_bacteria(x));
}
