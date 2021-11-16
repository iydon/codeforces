// https://codeforces.com/problemset/problem/71/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn main() {
    if let Ok(number) = input().parse::<i8>() {
        for _ in 0..number {
            let word = input();
            let length = word.len();
            if length > 10 {
                println!("{}{}{}", &word[..1], length-2, &word[length-1..]);
            } else {
                println!("{}", word);
            }
        }
    }
}
