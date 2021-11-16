// https://codeforces.com/problemset/problem/50/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let numbers: Vec<u8> = input()
        .split_whitespace().take(2)
        .map(|s| s.parse().unwrap())
        .collect();
    let m = numbers[0] as u16;
    let n = numbers[1] as u16;
    println!("{}", m*n/2);
}
