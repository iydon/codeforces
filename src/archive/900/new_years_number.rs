// https://codeforces.com/problemset/problem/1475/B
// https://codeforces.com/problemset/problem/1475/B
use std::collections::HashSet;

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

fn main() {
    let mut set: HashSet<i32> = HashSet::with_capacity(250000);
    for ith in 0..500 {
        for jth in 0..500 {
            let w = ith*2020 + jth*2021;
            if w > 1_000_000 {
                break;
            }
            set.insert(w);
        }
    }
    for _ in 0..input::scalar::<u16>() {
        println!("{}", if set.contains(&input::scalar()) {"YES"} else {"NO"});
    }
}
