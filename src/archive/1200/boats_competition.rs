// https://codeforces.com/problemset/problem/1399/C
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

    fn solve(&self, _n: usize, wn: Vec<usize>) -> u8 {
        let mut counter = [0; 51];
        let (mut min, mut max) = (0, 0);
        let (mut sum, mut ans) = (0, 0);
        for w in wn.into_iter() {
            min = min.min(w);
            max = max.max(w);
            counter[w] += 1;
        }
        for ith in 2 * min..=2 * max {
            for jth in min..=ith - min {
                match (counter.get(jth), counter.get(ith - jth)) {
                    (Some(a), Some(b)) => sum += a.min(b),
                    _ => {}
                };
            }
            ans = ans.max(sum);
            sum = 0;
        }
        return ans / 2;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let n = input.scalar();
            let wn = input.vector(n);
            writeln!(stdout, "{}", self.solve(n, wn)).unwrap();
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
        assert_eq!(crate::Problem::_test("5\n5\n1 2 3 4 5\n8\n6 6 6 6 6 6 8 8\n8\n1 2 2 1 2 1 1 2\n3\n1 3 3\n6\n1 1 3 4 2 2\n"), "2\n3\n4\n1\n2\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
