// https://codeforces.com/problemset/problem/791/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let ns: Vec<f64> = input()
        .split_whitespace().take(2)
        .map(|s| s.parse().unwrap())
        .collect();
    let ratio = f64::ln(ns[1]/ns[0]) / f64::ln(3.0/2.0);
    println!("{}", (ratio+1.0).floor());
}
