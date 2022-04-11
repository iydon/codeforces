// https://codeforces.com/problemset/problem/4/C
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
}

struct Problem {
    users: HashMap<String, u32>,
}

impl Problem {
    fn new() -> Self {
        return Problem {
            users: HashMap::new(),
        };
    }

    fn solve(&mut self, name: String) -> String {
        match self.users.get_mut(&name) {
            Some(ith) => {
                *ith += 1;
                return format!("{}{}", name, *ith);
            }
            None => {
                self.users.insert(name, 0);
                return "OK".to_string();
            }
        };
    }

    fn via_io<I, O>(mut self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u32>() {
            let ans = self.solve(input.text());
            writeln!(stdout, "{}", ans).unwrap();
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
            crate::Problem::_test("4\nabacaba\nacaba\nabacaba\nacab\n"),
            "OK\nOK\nabacaba1\nOK\n"
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            crate::Problem::_test("6\nfirst\nfirst\nsecond\nsecond\nthird\nthird\n"),
            "OK\nfirst1\nOK\nsecond1\nOK\nthird1\n"
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(crate::Problem::_test("3\nb\nb\nb\n"), "OK\nb1\nb2\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
