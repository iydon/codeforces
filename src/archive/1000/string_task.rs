// https://codeforces.com/problemset/problem/118/A
mod input {
    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }
}

fn main() {
    let word: String = input::raw();
    for c in word.trim().to_ascii_lowercase().chars() {
        match c {
            'a' | 'o' | 'e' | 'i' | 'y'| 'u'|'\r'|'\n' => continue,
            _ => print!(".{}", c),
        };
    }
}
