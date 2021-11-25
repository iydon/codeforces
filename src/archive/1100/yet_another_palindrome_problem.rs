// https://codeforces.com/problemset/problem/1324/B
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

    fn solve(&self, n: usize, a: Vec<u16>) -> bool {
        let mut counter = vec![[0, 0]; n];
        for ith in 1..=n {
            let jth = a[ith - 1] as usize - 1;
            match counter[jth] {
                [0, _] => counter[jth][0] = ith,
                [_, 0] => counter[jth][1] = ith,
                [_, _] => return true,
            };
        }
        for [c1, c2] in counter {
            if c2 != 0 && c1 + 1 < c2 {
                return true;
            }
        }
        return false;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u8>() {
            let n = input.scalar();
            match self.solve(n, input.vector(n)) {
                true => writeln!(stdout, "YES").unwrap(),
                false => writeln!(stdout, "NO").unwrap(),
            };
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
                "5\n3\n1 2 1\n5\n1 2 2 3 2\n3\n1 1 2\n4\n1 2 2 1\n10\n1 1 2 2 3 3 4 4 5 5\n"
            ),
            "YES\nYES\nNO\nYES\nNO\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    Problem::new().via_io(stdin.lock(), stdout.lock());
}
