// ? https://codeforces.com/problemset/problem/1354/B
use std::collections::HashMap;

pub struct Input<I: std::io::BufRead> {
    std: I,
    buffer: Vec<String>,
}

impl<I: std::io::BufRead> Input<I> {
    pub fn new(std: I) -> Self {
        return Self {
            std: std,
            buffer: Vec::new(),
        };
    }

    pub fn raw(&mut self) -> String {
        let mut string = String::new();
        self.std.read_line(&mut string).unwrap();
        return string;
    }

    pub fn text(&mut self) -> String {
        return self.raw().trim().to_string();
    }

    pub fn next(&mut self) -> String {
        loop {
            match self.buffer.pop() {
                Some(token) => return token,
                None => {
                    self.buffer = self
                        .raw()
                        .split_whitespace()
                        .rev()
                        .map(String::from)
                        .collect();
                }
            };
        }
    }

    pub fn scalar<T>(&mut self) -> T
    where
        T: std::str::FromStr,
    {
        return self.next().parse().ok().unwrap();
    }

    pub fn vector<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
    {
        return (0..n).map(|_| self.scalar()).collect();
    }
}

struct Problem {}

impl Problem {
    fn new() -> Self {
        return Problem {};
    }

    fn solve(&self, numbers: Vec<u8>) -> usize {
        let mut counter = HashMap::with_capacity(3);
        for number in numbers.iter() {
            *counter.entry(*number).or_insert(0) += 1;
        }
        if counter.len() != 3 {
            return 0;
        }
        let (mut left, mut right) = (0, numbers.len() - 1);
        macro_rules! select {
            (left, $n:ident) => {{
                *counter.entry($n).or_insert(0) -= 1;
                left += 1;
            }};
            (right, $n:ident) => {{
                *counter.entry($n).or_insert(0) -= 1;
                right -= 1;
            }};
        }
        while left + 2 < right {
            let (nl, nr) = (numbers[left], numbers[right]);
            match (counter[&nl] > 1, counter[&nr] > 1) {
                (true, true) => {
                    match counter[&nl] < counter[&nr] {
                        true => select!(right, nr),
                        false => select!(left, nl),
                    };
                }
                (true, false) => select!(left, nl),
                (false, true) => select!(right, nr),
                (false, false) => break,
            };
        }
        return right - left + 1;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let numbers = input.text().chars().map(|c| c as u8 - 48).collect();
            writeln!(stdout, "{}", self.solve(numbers)).unwrap();
        }
    }

    fn _test(input: &str) -> String {
        let mut output = Vec::new();
        let problem = Self::new();
        problem.via_io(input.as_bytes(), &mut output);
        return String::from_utf8(output).unwrap();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn case_1() {
        assert_eq!(
            crate::Problem::_test(
                "7\n123\n12222133333332\n112233\n332211\n12121212\n333333\n31121\n"
            ),
            "3\n3\n4\n4\n0\n0\n4\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
