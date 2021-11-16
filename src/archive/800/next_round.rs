// https://codeforces.com/problemset/problem/158/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    if let Some(raw) = input().split_whitespace().nth(1) {
        let k = raw.parse::<u8>().unwrap();
        let numbers: Vec<u8> = input()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let min = numbers[k as usize-1];
        let count = numbers
            .into_iter()
            .filter(|&n| n>=min && n>0)
            .count();
        println!("{}", count);
    }
}
