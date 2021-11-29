// https://codeforces.com/problemset/problem/1328/C
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
    fn solve(&self, n: usize, x: Vec<char>) -> (String, String) {
        let (mut min, mut max) = (vec!['1'; n], vec!['1'; n]);
        let mut antonym = false;
        for ith in 1..n {
            match x[ith] {
                '0' => {
                    min[ith] = '0';
                    max[ith] = '0';
                }
                '1' => {
                    if antonym {
                        min[ith] = '0';
                    } else {
                        max[ith] = '0';
                        antonym = true;
                    }
                }
                '2' => {
                    if antonym {
                        min[ith] = '0';
                        max[ith] = '2';
                    }
                }
                _ => unreachable!(),
            };
        }
        return (min.into_iter().collect(), max.into_iter().collect());
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let n = input.scalar();
            let x = input.text().chars().collect();
            let (a, b) = self.solve(n, x);
            writeln!(stdout, "{}\n{}", a, b).unwrap();
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
            crate::Problem::_test("4\n5\n22222\n5\n21211\n1\n2\n9\n220222021\n"),
            "11111\n11111\n11000\n10211\n1\n1\n110111011\n110111010\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
