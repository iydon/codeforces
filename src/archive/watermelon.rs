// https://codeforces.com/problemset/problem/4/A
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let w: i8 = line.trim().parse().unwrap_or(0);
    println!("{}", if w>=4 && w%2==0 {"YES"} else {"NO"});
}
