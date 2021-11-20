// https://codeforces.com/problemset/problem/479/A
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

fn expression(a: u16, b: u16, c: u16) -> u16 {
    return (a + b + c)
        .max(a + b * c)
        .max((a + b) * c)
        .max(a * b + c)
        .max(a * (b + c))
        .max(a * b * c);
}

fn main() {
    let (a, b, c) = (input::scalar(), input::scalar(), input::scalar());
    println!("{}", expression(a, b, c));
}
