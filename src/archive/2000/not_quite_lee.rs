// ? https://codeforces.com/contest/1610/problem/D
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

    fn solve(&self, n: usize, an: Vec<i64>) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let mut pow2 = vec![1; n + 1];
        let mut count = [0; 31];
        let mut sum = 0;
        for ith in 0..n {
            pow2[ith + 1] = pow2[ith] * 2 % MOD;
        }
        for a in an {
            if a % 2 == 0 {
                count[a.trailing_zeros() as usize - 1] += 1;
            }
        }
        let mut ans = pow2[n] - 1;
        for ith in (0..31).rev() {
            if count[ith] != 0 {
                sum += count[ith];
                ans += MOD - pow2[sum - 1];
            }
        }
        return ans % MOD;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let n = input.scalar();
        let an = input.vector(n);
        writeln!(stdout, "{}", self.solve(n, an)).unwrap();
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
        assert_eq!(crate::Problem::_test("4\n2 2 4 7\n"), "10\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("10\n12391240 103904 1000000000 4142834 12039 142035823 1032840 49932183 230194823 984293123\n"), "996\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
