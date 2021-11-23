// https://codeforces.com/problemset/problem/313/B
mod input {
    use std::str::FromStr;

    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }

    pub fn text() -> String {
        return raw().trim().to_string();
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
    // https://codeforces.com/problemset/submission/313/55668736
    let line = input::text();
    let diff = {
        let mut diff = Vec::with_capacity(line.len());
        diff.push(0);
        line.chars()
            .zip(line.chars().skip(1))
            .map(|(a, b)| if a == b { 1 } else { 0 })
            .for_each(|x| {
                let x = x + diff.last().unwrap();
                diff.push(x)
            });
        diff
    };
    for _ in 0..input::scalar::<u32>() {
        let lr: Vec<usize> = input::vector(2);
        println!("{}", diff[lr[1] - 1] - diff[lr[0] - 1]);
    }
}
