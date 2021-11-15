// https://codeforces.com/problemset/problem/112/A
use std::cmp::Ordering;


fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let first = input().to_ascii_lowercase();
    let second = input().to_ascii_lowercase();
    let flag: i8 = match (&first).cmp(&second) {
        Ordering::Equal   =>  0,
        Ordering::Less    => -1,
        Ordering::Greater =>  1,
    };
    println!("{}", flag);
}
