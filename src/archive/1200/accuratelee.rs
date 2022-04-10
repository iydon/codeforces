// https://codeforces.com/problemset/problem/1369/B
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
}

struct Problem {}

impl Problem {
    fn new() -> Self {
        return Problem {};
    }

    fn solve(&self, s: Vec<bool>, n: usize) -> String {
        // https://codeforces.com/contest/1369/submission/140961052
        let mut ans = String::new();
        let mut ith = 0;
        let mut jth = n;
        while ith < n && !s[ith] {
            ith += 1;
        }
        while jth > 0 && s[jth - 1] {
            jth -= 1;
        }
        for _ in 0..ith {
            ans.push('0');
        }
        if ith < jth {
            ans.push('0');
        }
        for _ in 0..(n - jth) {
            ans.push('1');
        }
        return ans;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let n = input.scalar();
            let s = input
                .text()
                .chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => unreachable!(),
                })
                .collect();
            writeln!(stdout, "{}", self.solve(s, n)).unwrap();
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
                "5\n10\n0001111111\n4\n0101\n8\n11001101\n10\n1110000000\n1\n1\n"
            ),
            "0001111111\n001\n01\n0\n1\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
