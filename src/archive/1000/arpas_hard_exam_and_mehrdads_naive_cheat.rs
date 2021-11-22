// https://codeforces.com/problemset/problem/742/A
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

fn arpas_hard_exam_and_mehrdads_naive_cheat(n: u32) -> u8 {
    if n == 0 {
        return 1;
    }
    return match n % 4 {
        0 => 6,
        1 => 8,
        2 => 4,
        3 => 2,
        _ => panic!(),
    };
}

fn main() {
    let n: u32 = input::scalar();
    println!("{}", arpas_hard_exam_and_mehrdads_naive_cheat(n));
}
