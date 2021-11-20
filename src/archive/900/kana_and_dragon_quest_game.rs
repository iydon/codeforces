// https://codeforces.com/problemset/problem/1337/B
// https://codeforces.com/problemset/page/5?order=BY_RATING_ASC
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

fn kana_and_dragon_quest_game(mut x: u32, n: u32, m: u32) -> bool {
    for _ in 0..n {
        x = u32::min(x, x/2 + 10);
    }
    return x<=10*m;
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        let xnm = input::vector(3);
        match kana_and_dragon_quest_game(xnm[0], xnm[1], xnm[2]) {
            true => println!("YES"),
            false => println!("NO"),
        };
    }
}
