// https://codeforces.com/problemset/problem/231/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    if let Ok(number) = input().parse::<u16>() {
        let mut answer: u16 = 0;
        for _ in 0..number {
            let sum: u8 = input()
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .sum();
            if sum >= 2 { answer += 1; }
        }
        println!("{}", answer);
    }
}
