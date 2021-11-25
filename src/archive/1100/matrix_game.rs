// https://codeforces.com/problemset/problem/1365/A
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

    fn solve(&self, n: usize, m: usize, checkerboard: Vec<Vec<bool>>) -> bool {
        let (mut rows, mut cols) = (vec![true; n], vec![true; m]);
        for ith in 0..n {
            for jth in 0..m {
                if checkerboard[ith][jth] {
                    rows[ith] = false;
                    cols[jth] = false;
                }
            }
        }
        let turns = usize::min(
            rows.into_iter().map(|x| x as usize).sum(),
            cols.into_iter().map(|x| x as usize).sum(),
        );
        return turns % 2 == 0;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u8>() {
            let (n, m) = (input.scalar(), input.scalar());
            let checkerboard = (0..n)
                .map(|_| {
                    input
                        .vector::<char>(m)
                        .into_iter()
                        .map(|e| match e {
                            '0' => false,
                            '1' => true,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect();
            match self.solve(n, m, checkerboard) {
                true => writeln!(stdout, "Vivek").unwrap(),
                false => writeln!(stdout, "Ashish").unwrap(),
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
        assert_eq!(
            crate::Problem::_test(
                "4\n2 2\n0 0\n0 0\n2 2\n0 0\n0 1\n2 3\n1 0 1\n1 1 0\n3 3\n1 0 0\n0 0 0\n1 0 0\n"
            ),
            "Vivek\nAshish\nVivek\nAshish\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
