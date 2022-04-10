// https://codeforces.com/problemset/problem/514/A
pub struct Input<I: std::io::BufRead> {
    std: I,
}

impl<I: std::io::BufRead> Input<I> {
    pub fn new(std: I) -> Self {
        return Self { std: std };
    }

    pub fn raw(&mut self) -> String {
        let mut string = String::new();
        self.std.read_line(&mut string).unwrap();
        return string;
    }

    pub fn text(&mut self) -> String {
        return self.raw().trim().to_string();
    }
}

struct Problem {}

impl Problem {
    fn new() -> Self {
        return Problem {};
    }

    fn solve(&self, number: Vec<char>) -> String {
        let mut ans = String::new();
        ans.push(match number[0] {
            '0' | '1' | '2' | '3' | '4' | '9' => number[0],
            _ => (105 - number[0] as u8) as char,
        });
        for chr in number.into_iter().skip(1) {
            ans.push(match chr {
                '0' | '1' | '2' | '3' | '4' => chr,
                _ => (105 - chr as u8) as char,
            });
        }
        return ans;
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        writeln!(stdout, "{}", self.solve(input.text().chars().collect())).unwrap();
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
        assert_eq!(crate::Problem::_test("27\n"), "22\n");
    }

    #[test]
    fn case_2() {
        assert_eq!(crate::Problem::_test("4545\n"), "4444\n");
    }

    #[test]
    fn case_3() {
        assert_eq!(crate::Problem::_test("9\n"), "9\n");
    }

    #[test]
    fn case_4() {
        assert_eq!(crate::Problem::_test("91730629\n"), "91230320\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
