// https://codeforces.com/problemset/problem/282/A
fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

fn main() {
    if let Ok(number) = input().trim().parse::<u8>() {
        let sum: i16 = (0..number)
            .map(|_| if input().contains("+") {1} else {-1})
            .sum();
        println!("{}", sum);
    }
}
