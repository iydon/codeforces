// https://codeforces.com/problemset/problem/489/B
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

    fn solve(&self, n: usize, m: usize, mut boys: Vec<u8>, mut girls: Vec<u8>) -> u8 {
        boys.sort();
        girls.sort();
        let mut ans = 0;
        let (mut ith, mut jth) = (0, 0);
        while ith < n && jth < m {
            if boys[ith] <= girls[jth] + 1 && girls[jth] <= boys[ith] + 1 {
                ith += 1;
                jth += 1;
                ans += 1;
            } else if boys[ith] > girls[jth] {
                jth += 1;
            } else {
                ith += 1;
            }
        }
        return ans;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let n = input.scalar();
        let boys = input.vector(n);
        let m = input.scalar();
        let girls = input.vector(m);
        let ans = self.solve(n, m, boys, girls);
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
        assert_eq!(crate::Problem::_test("4\n1 4 6 2\n5\n5 1 5 7 9\n"), "3\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("4\n1 2 3 4\n4\n10 11 12 13\n"), "0\n");
    }

    #[test]
    fn case_3() {
        assert_eq!(crate::Problem::_test("5\n1 1 1 1 1\n3\n1 2 3\n"), "2\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
