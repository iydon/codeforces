// https://codeforces.com/problemset/problem/236/A
use std::collections::HashSet;


fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let username = input();
    let mut set = HashSet::<char>::new();
    for c in username.chars() {
        set.insert(c);
    }
    match set.len()%2 {
        0 => println!("CHAT WITH HER!"),
        1 => println!("IGNORE HIM!"),
        _ => (),
    };
}
