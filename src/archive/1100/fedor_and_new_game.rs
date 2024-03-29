// https://codeforces.com/problemset/problem/467/B
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

    fn bitsum(&self, n: u8, mut x: u32) -> u8 {
        let mut sum = 0;
        for _ in 0..n {
            sum += (x % 2) as u8;
            x /= 2;
        }
        return sum;
    }

    fn solve(&self, n: u8, m: u16, k: u8, xs: Vec<u32>) -> u16 {
        let mut ans = 0;
        let last = xs.last().unwrap();
        for x in xs.iter().take(m as usize) {
            if self.bitsum(n, x ^ last) <= k {
                ans += 1;
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
        let (n, m, k) = (input.scalar(), input.scalar(), input.scalar());
        let xs: Vec<u32> = input.vector(1 + m as usize);
        let ans = self.solve(n, m, k, xs);
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
        assert_eq!(crate::Problem::_test("7 3 1\n8\n5\n111\n17\n"), "0\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("3 3 3\n1\n2\n3\n4\n"), "3\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    Problem::new().via_io(stdin.lock(), stdout.lock());
}
