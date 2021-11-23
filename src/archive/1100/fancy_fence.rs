// https://codeforces.com/problemset/problem/270/A
// ```python3
// for a in range(1, 180):
//     if 360 % (180-a) == 0:
//         print(a)
// ```
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
    for _ in 0..input::scalar::<u8>() {
        match input::raw().trim() {
            "60" | "90" | "108" | "120" | "135" | "140" | "144" | "150" | "156" | "160" | "162"
            | "165" | "168" | "170" | "171" | "172" | "174" | "175" | "176" | "177" | "178"
            | "179" => println!("YES"),
            _ => println!("NO"),
        };
    }
}
