// https://codeforces.com/problemset/problem/474/A
use std::collections::HashMap;

mod input {
    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }
}

fn main() {
    const K: &[u8] = b"qwertyuiopasdfghjkl;zxcvbnm,./";
    let map = match input::raw().trim() {
        "L" => K.iter().zip(K.iter().skip(1)).collect::<HashMap<_, _>>(),
        "R" => K.iter().skip(1).zip(K.iter()).collect::<HashMap<_, _>>(),
        _ => panic!(),
    };
    for c in input::raw().trim().chars() {
        print!("{}", *map[&(c as u8)] as char);
    }
    println!();
}
