// https://codeforces.com/problemset/problem/459/A
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

    #[inline]
    fn solve(&self, x1: i16, y1: i16, x2: i16, y2: i16) -> Option<(i16, i16, i16, i16)> {
        match (x2 - x1, y2 - y1) {
            (dx, 0) if dx != 0 => return Some((x1, y1 + dx, x2, y2 + dx)),
            (0, dy) if dy != 0 => return Some((x1 + dy, y1, x2 + dy, y2)),
            (dx, dy) if dx.abs() == dy.abs() => return Some((x1, y2, x2, y1)),
            _ => return None,
        }
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let (x1, y1) = (input.scalar(), input.scalar());
        let (x2, y2) = (input.scalar(), input.scalar());
        match self.solve(x1, y1, x2, y2) {
            Some((x1, y1, x2, y2)) => {
                writeln!(stdout, "{} {} {} {}", x1, y1, x2, y2).unwrap();
            }
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
        assert_eq!(crate::Problem::_test("0 0 0 1\n"), "1 0 1 1\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("0 0 1 1\n"), "0 1 1 0\n");
    }

    #[test]
    fn case_3() {
        assert_eq!(crate::Problem::_test("0 0 1 2\n"), "-1\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
