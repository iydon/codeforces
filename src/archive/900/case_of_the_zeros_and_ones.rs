// https://codeforces.com/problemset/problem/556/A
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
    let _ = input::raw();
    let sum: i32 = input::text().chars().map(|c| match c {
        '0' =>  1i32,
        _   => -1i32,
    }).sum();
    println!("{}", sum.abs());
}
