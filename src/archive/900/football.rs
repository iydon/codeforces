// https://codeforces.com/problemset/problem/96/A
mod input {
    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }

    pub fn text() -> String {
        return raw().trim().to_string();
    }
}

fn main() {
    let text = input::text();
    if text.contains("1111111") || text.contains("0000000") {
        println!("YES");
    } else {
        println!("NO");
    }
}
