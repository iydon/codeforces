// https://codeforces.com/problemset/problem/313/A
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
    if !text.contains("-") {
        println!("{}", text);
    } else {
        let end = text.len();
        let n1: i32 = (&text[..end - 1]).parse().unwrap();
        let n2: i32 = (format!("{}{}", &text[..end - 2], &text[end - 1..]))
            .parse()
            .unwrap();
        println!("{}", if n1 > n2 { n1 } else { n2 });
    }
}
