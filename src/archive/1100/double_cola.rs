// https://codeforces.com/problemset/problem/82/A
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

    fn solve(&self, n: u32) -> u32 {
        // log2(n/5+1)-1 <= k < log2(n/5+1)
        let k = (((n as f32) / 5.0 + 1.0).log2() - 1.0).ceil() as u32;
        let min = 5 * (u32::pow(2, k) - 1) + 1;
        return (n - min) / u32::pow(2, k);
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let n: u32 = input.scalar();
        let ans = self.solve(n);
        let name = match ans {
            0 => "Sheldon",
            1 => "Leonard",
            2 => "Penny",
            3 => "Rajesh",
            4 => "Howard",
            _ => unreachable!(),
        };
        writeln!(stdout, "{}", name).unwrap();
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
        assert_eq!(crate::Problem::_test("1\n"), "Sheldon\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("6\n"), "Sheldon\n");
    }

    #[test]
    fn case_3() {
        assert_eq!(crate::Problem::_test("1802\n"), "Penny\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    Problem::new().via_io(stdin.lock(), stdout.lock());
}
