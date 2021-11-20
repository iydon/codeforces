// https://codeforces.com/problemset/problem/58/A
mod input {
    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }
}

fn main() {
    let mut hello = "hello".chars().peekable();
    for c in input::raw().trim().chars() {
        if Some(&c) == hello.peek() {
            hello.next();
        }
    }
    println!("{}", if hello.peek()==None {"YES"} else {"NO"} );
}
