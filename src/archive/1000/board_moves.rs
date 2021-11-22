// https://codeforces.com/problemset/problem/1353/C
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

fn board_moves(n: u64) -> u64 {
    return (n - 1) * n * (n + 1) / 3;
}

fn main() {
    for _ in 0..input::scalar::<u8>() {
        let n: u64 = input::scalar();
        // assert_eq!(n%2, 1);
        println!("{}", board_moves(n));
    }
}
