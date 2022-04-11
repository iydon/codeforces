// https://codeforces.com/problemset/problem/230/B
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

struct Problem {
    cache: Vec<bool>,
}

impl Problem {
    fn new() -> Self {
        let mut cache = vec![false; 1_000_001];
        cache[0] = true;
        cache[1] = true;
        for ith in 2..cache.len() {
            if !cache[ith] {
                for jth in (ith * ith..cache.len()).step_by(ith) {
                    cache[jth] = true;
                }
            }
        }
        return Problem { cache };
    }

    fn solve(&self, x: u64) -> bool {
        let root = (x as f64).sqrt();
        if root.fract() == 0.0 && root as u8 != 1 {
            return !self.cache[root as usize];
        } else {
            return false;
        }
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u32>() {
            match self.solve(input.scalar()) {
                true => writeln!(stdout, "YES").unwrap(),
                false => writeln!(stdout, "NO").unwrap(),
            }
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
        assert_eq!(crate::Problem::_test("3\n4 5 6\n"), "YES\nNO\nNO\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
