use std::convert::TryInto;
use std::io::Write;
use std::str::FromStr;

#[allow(dead_code)]
pub fn prompt(text: &str) -> () {
    print!("{}", text);
    std::io::stdout().flush().unwrap();
}

#[allow(dead_code)]
pub fn raw() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

#[allow(dead_code)]
pub fn text() -> String {
    return raw().trim().to_string();
}

#[allow(dead_code)]
pub fn scalar<T>() -> T
where
    T: FromStr,
{
    return raw().trim().parse().unwrap_or_else(|_| panic!());
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn array<T, const N: usize>() -> [T; N]
where
    T: FromStr,
{
    // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    return vector(N).try_into().unwrap_or_else(|_| panic!());
}
