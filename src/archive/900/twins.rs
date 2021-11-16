// https://codeforces.com/problemset/problem/160/A
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

    pub fn vector<T>(length: usize) -> Vec<T>
    where
        T: FromStr,
    {
        return raw()
            .trim()
            .split_whitespace()
            .take(length)
            .map(|s| s.parse().unwrap_or_else(|_| panic!()))
            .collect();
    }
}

fn main() {
    let mut ns: Vec<u16> = input::vector(input::scalar());
    ns.sort_by(|x, y| y.cmp(x));
    let sum: u16 = ns.iter().sum();
    let mut mine = 0;
    let mut answer = 0;
    for n in ns {
        answer += 1;
        mine += n;
        if 2 * mine > sum {
            break;
        }
    }
    println!("{}", answer);
}
