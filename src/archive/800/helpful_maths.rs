// https://codeforces.com/problemset/problem/339/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let mut numbers: Vec<u8> = input()
        .split("+")
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort();
    let statement: Vec<String> = numbers.iter()
        .map(|n| n.to_string())
        .collect();
    println!("{}", statement.join("+"));
}
