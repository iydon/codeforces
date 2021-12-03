// https://codeforces.com/problemset/problem/433/B
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

struct Problem {
    vn: Vec<usize>,
    un: Vec<usize>,
}

impl Problem {
    fn new() -> Self {
        return Problem {
            vn: vec![],
            un: vec![],
        };
    }

    fn solve(&self, t: bool, l: usize, r: usize) -> usize {
        let xn = if t { &self.vn } else { &self.un };
        return xn[r - 1] - if l == 1 { 0 } else { xn[l - 2] };
    }

    fn via_io<I, O>(mut self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let n = input.scalar();
        self.vn = input.vector(n);
        self.un = self.vn.clone();
        self.un.sort();
        for ith in 1..n {
            self.vn[ith] += self.vn[ith - 1];
            self.un[ith] += self.un[ith - 1];
        }
        for _ in 0..input.scalar::<u32>() {
            let ans = self.solve(input.next().eq("1"), input.scalar(), input.scalar());
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
            crate::Problem::_test("6\n6 4 2 7 2 7\n3\n2 3 6\n1 3 4\n1 1 6\n"),
            "24\n9\n28\n"
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("4\n5 5 2 3\n10\n1 2 4\n2 1 4\n1 1 1\n2 1 4\n2 1 2\n1 1 1\n1 3 3\n1 1 3\n1 4 4\n1 2 2\n"), "10\n15\n5\n15\n5\n5\n2\n12\n3\n5\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
