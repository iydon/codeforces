// https://codeforces.com/problemset/problem/1472/C
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

    fn solve(&self, mut a: Vec<u32>) -> u32 {
        let length = a.len();
        let mut max = 0;
        for ith in (0..length).rev() {
            let jth = ith + a[ith] as usize;
            if jth < length {
                a[ith] += a[jth];
            }
            max = max.max(a[ith]);
        }
        return max;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let n = input.scalar();
            let ans = self.solve(input.vector(n));
            writeln!(stdout, "{}", ans).unwrap();
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
            crate::Problem::_test("4\n5\n7 3 1 2 3\n3\n2 1 4\n6\n2 1000 2 3 995 1\n5\n1 1 1 1 1\n"),
            "7\n6\n1000\n5\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    Problem::new().via_io(stdin.lock(), stdout.lock());
}
