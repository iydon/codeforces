// https://codeforces.com/problemset/problem/977/C
use std::cmp::Ordering;
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

    fn solve(&self, an: Vec<u32>, k: u32) -> Option<u32> {
        let mut counter = HashMap::new();
        let mut sorted: Vec<_>;
        let mut sum = 0;
        for a in an.into_iter() {
            if a <= 1_000_000_000 {
                *counter.entry(a).or_insert(0) += 1;
            }
        }
        if k == 0 {
            return if counter.contains_key(&1) {
                None
            } else {
                Some(1)
            };
        }
        sorted = counter.into_iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, value) in sorted.into_iter() {
            sum += value;
            match sum.cmp(&k) {
                Ordering::Greater => break,
                Ordering::Equal => return Some(key),
                Ordering::Less => (),
            }
        }
        return None;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let (n, k) = (input.scalar(), input.scalar());
        let an = input.vector(n);
        match self.solve(an, k) {
            Some(x) => writeln!(stdout, "{}", x).unwrap(),
            None => writeln!(stdout, "-1").unwrap(),
        };
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
        assert_eq!(crate::Problem::_test("7 4\n3 7 5 1 10 3 20\n"), "5\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("7 2\n3 7 5 1 10 3 20\n"), "-1\n");
    }

    #[test]
    fn case_3() {
        assert_eq!(crate::Problem::_test("1 0\n2\n"), "1\n");
    }

    #[test]
    fn case_4() {
        assert_eq!(crate::Problem::_test("1 1\n1000000000\n"), "1000000000\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
