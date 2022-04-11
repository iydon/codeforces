// https://codeforces.com/problemset/problem/189/A
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
}

struct Problem {}

impl Problem {
    fn new() -> Self {
        return Problem {};
    }

    fn solve(&self, n: usize, a: usize, b: usize, c: usize) -> usize {
        let mut z = [0; 5_001];
        z[0] = 1;
        for ith in 1..n + 1 {
            if ith >= a && z[ith - a] != 0 && z[ith - a] + 1 > z[ith] {
                z[ith] = z[ith - a] + 1;
            }
            if ith >= b && z[ith - b] != 0 && z[ith - b] + 1 > z[ith] {
                z[ith] = z[ith - b] + 1
            }
            if ith >= c && z[ith - c] != 0 && z[ith - c] + 1 > z[ith] {
                z[ith] = z[ith - c] + 1
            }
        }
        return z[n] - 1;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let ans = self.solve(
            input.scalar(),
            input.scalar(),
            input.scalar(),
            input.scalar(),
        );
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
        assert_eq!(crate::Problem::_test("5 5 3 2\n"), "2\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("7 5 5 2\n"), "2\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
