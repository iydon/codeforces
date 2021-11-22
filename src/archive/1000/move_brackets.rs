// https://codeforces.com/problemset/problem/1374/C
mod input {
    use std::str::FromStr;

    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }

    pub fn scalar<T>() -> T
    where
        T: FromStr,
    {
        return raw().trim().parse().unwrap_or_else(|_| panic!());
    }
}

fn move_brackets(text: &str) -> u8 {
    let mut ans = 0;
    let mut stack = 0;
    for c in text.chars() {
        match c {
            '(' => {
                stack += 1;
            }
            ')' => {
                if stack == 0 {
                    ans += 1;
                } else {
                    stack -= 1;
                }
            }
            _ => {},
        }
    }
    return ans;
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        let _ = input::raw();
        println!("{}", move_brackets(input::raw().trim()));
    }
}
