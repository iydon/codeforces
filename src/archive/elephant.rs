// https://codeforces.com/problemset/problem/617/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let x: u32 = input().parse().unwrap();
    let answer = x/5 + ((x%5)!=0) as u32;
    println!("{}", answer);
}
