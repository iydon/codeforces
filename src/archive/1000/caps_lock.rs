// https://codeforces.com/problemset/problem/131/A
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
    let word = input::text();
    if word.chars().skip(1).all(|c| c.is_ascii_uppercase()) {
        let first = word.chars().next().unwrap();
        print!(
            "{}",
            match first.is_ascii_uppercase() {
                true => first.to_ascii_lowercase(),
                false => first.to_ascii_uppercase(),
            }
        );
        println!("{}", (&word[1..]).to_ascii_lowercase());
    } else {
        println!("{}", word);
    }
}
