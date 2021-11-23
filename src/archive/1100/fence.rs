// https://codeforces.com/problemset/problem/363/B
mod input {
    use std::str::FromStr;

    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
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
    let nk: Vec<usize> = input::vector(2);
    let hs: Vec<u32> = input::vector(nk[0]);
    let mut sum: u32 = hs.iter().take(nk[1]).sum();
    let (mut jth, mut min) = (0, sum);
    for ith in 1..(hs.len() - nk[1] + 1) {
        sum = sum + hs[nk[1] + ith - 1] - hs[ith - 1];
        if sum < min {
            min = sum;
            jth = ith;
        }
    }
    println!("{}", jth + 1);
}
