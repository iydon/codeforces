// https://codeforces.com/problemset/problem/1327/A
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

fn sum_of_odd_integers(n: u64, k: u64) -> bool {
    return !(n%2 != k%2 || n < k*k);
    // return if n%2 != k%2 {
    //     false
    // } else if n < k*k {
    //     false
    // } else {
    //     true
    // };
}

fn main() {
    for _ in 0..input::scalar::<u32>() {
        let nk: Vec<u64> = input::vector(2);
        match sum_of_odd_integers(nk[0], nk[1]) {
            true => println!("YES"),
            false => println!("NO"),
        };
    }
}
