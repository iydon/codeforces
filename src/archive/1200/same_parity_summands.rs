// https://codeforces.com/problemset/problem/1352/B
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

    fn solve(&self, n: i32, k: i32) -> Option<Vec<i32>> {
        let x = n - k + 1;
        if x > 0 && x % 2 == 1 {
            let mut ans = vec![1; k as usize];
            ans[0] = x;
            return Some(ans);
        }
        let y = n - 2 * k + 2;
        if y > 0 && y % 2 == 0 {
            let mut ans = vec![2; k as usize];
            ans[0] = y;
            return Some(ans);
        }
        return None;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let (n, k) = (input.scalar(), input.scalar());
            match self.solve(n, k) {
                Some(ak) => {
                    let strings: Vec<String> = ak.iter().map(|a| a.to_string()).collect();
                    writeln!(stdout, "YES").unwrap();
                    writeln!(stdout, "{}", strings.join(" ")).unwrap();
                }
                None => writeln!(stdout, "NO").unwrap(),
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
        assert_eq!(crate::Problem::_test("8\n10 3\n100 4\n8 7\n97 2\n8 8\n3 10\n5 3\n1000000000 9\n"), "YES\n6 2 2\nYES\n97 1 1 1\nNO\nNO\nYES\n1 1 1 1 1 1 1 1\nNO\nYES\n3 1 1\nYES\n999999984 2 2 2 2 2 2 2 2\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
