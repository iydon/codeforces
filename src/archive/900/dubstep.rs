// https://codeforces.com/problemset/problem/208/A
mod input {
    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }
}

fn main() {
    let text = input::raw()
        .replace("WUB", " ")
        .replace("  ", " ")
        .trim()
        .to_string();
    println!("{}", text);
}
