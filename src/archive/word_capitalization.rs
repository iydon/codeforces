// https://codeforces.com/problemset/problem/281/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn capitalization(word: String) -> String {
    let mut cs = word.chars();
    return match cs.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + cs.as_str()
    };
}

fn main() {
    println!("{}", capitalization(input()));
}
