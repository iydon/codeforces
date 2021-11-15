// https://codeforces.com/problemset?order=BY_RATING_ASC
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    println!("{}", input());
}
