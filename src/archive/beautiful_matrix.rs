// https://codeforces.com/problemset/problem/263/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    for ith in 0..5 {
        if let Some(jth) = input()
                .split_whitespace()
                .position(|s| s=="1") {
            println!(
                "{}", i8::abs(ith as i8-2)+i8::abs(jth as i8-2)
            );
            break;
        }
    }
}
