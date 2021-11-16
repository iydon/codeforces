// https://codeforces.com/problemset/problem/266/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    let _ = input();
    let colors = input();
    let mut ith: usize = 0;
    let mut answer: u8 = 0;
    while ith < colors.len()-1 {
        if &colors[ith..ith+1] == &colors[ith+1..ith+2] {
            answer += 1;
        }
        ith += 1;
    }
    println!("{}", answer);
}
