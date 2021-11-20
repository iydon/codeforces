// https://codeforces.com/problemset/problem/230/A
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

struct Dragon {
    strength: u32,
    bonus: u32,
}

fn main() {
    let sn: Vec<_> = input::vector(2);
    let mut s = sn[0];
    let mut dragons: Vec<Dragon> = Vec::with_capacity(sn[1] as usize);
    for _ in 0..sn[1] {
        let xy: Vec<u32> = input::vector(2);
        dragons.push(Dragon {
            strength: xy[0],
            bonus: xy[1],
        });
    }
    dragons.sort_by(|x, y| x.strength.cmp(&y.strength));
    for dragon in dragons {
        if dragon.strength >= s {
            println!("NO");
            return;
        }
        s += dragon.bonus;
    }
    println!("YES");
}
