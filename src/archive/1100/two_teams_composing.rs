// https://codeforces.com/problemset/problem/1335/C
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
    for _ in 0..input::scalar::<u16>() {
        let n: usize = input::scalar();
        let mut map = vec![0; n];
        let mut distinct = 0;
        for number in input::raw().trim().split_whitespace() {
            let a = number.parse::<usize>().unwrap() - 1;
            if map[a] == 0 {
                distinct += 1;
            }
            map[a] += 1;
        }
        let max = map.into_iter().max().unwrap();
        if max < distinct {
            println!("{}", max);
        } else {
            println!("{}", distinct.min(max - 1));
        }
    }
}
