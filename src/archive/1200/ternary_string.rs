// https://codeforces.com/problemset/problem/1354/B
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

    fn solve(&self, numbers: Vec<usize>) -> usize {
        let mut count = usize::MAX;
        let mut left = 0;
        let mut index = [0, 0, 0];
        for right in 0..numbers.len() {
            index[numbers[right]] += 1;
            while index[numbers[left]] > 1 {
                index[numbers[left]] -= 1;
                left += 1;
            }
            if index[0] != 0 && index[1] != 0 && index[2] != 0 {
                count = count.min(right - left + 1);
            }
        }
        return if count == usize::MAX { 0 } else { count };
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let numbers = input.text().chars().map(|c| c as usize - 49).collect();
            writeln!(stdout, "{}", self.solve(numbers)).unwrap();
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
                "7\n123\n12222133333332\n112233\n332211\n12121212\n333333\n31121\n"
            ),
            "3\n3\n4\n4\n0\n0\n4\n"
        );
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
