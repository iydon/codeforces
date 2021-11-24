// https://codeforces.com/problemset/problem/1360/C
// https://codeforces.com/blog/entry/67391
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

    fn solve(&self, mut a: Vec<u8>) -> bool {
        let parity: u8 = a.iter().map(|a| a % 2).sum();
        if parity % 2 == 0 {
            return true;
        } else {
            a.sort();
            return a.windows(2).any(|ab| ab[1] == ab[0] + 1);
        }
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar() {
            let n = input.scalar();
            let a: Vec<u8> = input.vector(n);
            match self.solve(a) {
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
        assert_eq!(crate::Problem::_test("7\n4\n11 14 16 12\n2\n1 8\n4\n1 1 1 1\n4\n1 2 5 6\n2\n12 13\n6\n1 6 3 10 5 8\n6\n1 12 3 10 5 8\n"), "YES\nNO\nYES\nYES\nYES\nYES\nNO\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("1\n4\n1 1 2 5\n"), "YES\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    Problem::new().via_io(stdin.lock(), stdout.lock());
}
