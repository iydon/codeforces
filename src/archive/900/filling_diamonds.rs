// https://codeforces.com/problemset/problem/1339/A
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

// fn filling_diamonds(n: u32) -> u32 {
//     // a_{1} = 1
//     // a_{n+1} = 1 + a_{n}
//     return n;
// }

fn main() {
    for _ in 0..input::scalar::<u16>() {
        println!("{}", input::scalar::<u32>());
    }
}
