// https://codeforces.com/problemset/problem/1374/B
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

fn multiply_by_2_divide_by_6(mut n: u32) -> Option<u32> {
    let (mut n2, mut n3) = (0, 0);
    while n%2 == 0 {
        n2 += 1;
        n /= 2;
    }
    while n%3 == 0 {
        n3 += 1;
        n /= 3;
    }
    if n==1 && n3>=n2 {
        return Some(2 * n3 - n2);
    } else {
        return None;
    }
}

fn main() {
    for _ in 0..input::scalar::<u16>() {
        match multiply_by_2_divide_by_6(input::scalar()) {
            Some(x) => println!("{}", x),
            None    => println!("-1"),
        };
    }
}
