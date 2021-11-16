// https://codeforces.com/problemset/problem/546/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let ns: Vec<i32> = input()
        .split_whitespace().take(3)
        .map(|s| s.parse().unwrap())
        .collect();
    let k = ns[0];
    let n = ns[1];
    let w = ns[2];
    let s = (k+w*k)*w/2;
    // match n >= s {
    //     true => println!("0"),
    //     false => println!("{}", s-n),
    // };
    println!("{}", std::cmp::max(0, s-n));
}
