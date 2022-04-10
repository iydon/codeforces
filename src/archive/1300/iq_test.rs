// https://codeforces.com/problemset/problem/25/A
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

    fn solve(&self, a: Vec<u8>, n: usize) -> usize {
        let mut even = 0;
        let mut odd = 0;
        let mut last_even = 1;
        let mut last_odd = 1;

        for ith in 0..n {
            if a[ith] % 2 == 0 {
                even += 1;
                last_even = ith;
            } else {
                odd += 1;
                last_odd = ith;
            }

            if even > 1 && odd == 1 {
                return last_odd + 1;
            } else if even == 1 && odd > 1 {
                return last_even + 1;
            }
        }
        return 0;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let n = input.scalar();
        let an = input.vector(n);
        let ans = self.solve(an, n);
        writeln!(stdout, "{}", ans).unwrap();
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
        assert_eq!(crate::Problem::_test("5\n2 4 7 8 10\n"), "3\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("4\n1 2 1 1\n"), "2\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
