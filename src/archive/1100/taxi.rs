// https://codeforces.com/problemset/problem/158/B
mod input {
    pub fn raw() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
    }
}

fn taxi(c1: i32, c2: i32, c3: i32, c4: i32) -> i32 {
    let c1_ = 0.max(c1 - c3 - c2%2*2);
    return c4 + c3 + (c2+1)/2 + (c1_+3)/4;
}

fn main() {
    let _ = input::raw();
    let (mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0);
    for s in input::raw().chars() {
        match s {
            '1' => c1 += 1,
            '2' => c2 += 1,
            '3' => c3 += 1,
            '4' => c4 += 1,
            _   => {},
        };
    }
    println!("{}", taxi(c1, c2, c3, c4));
}
