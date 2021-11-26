#[allow(dead_code)]
mod template;
use template::input::Input;

struct Problem {
    numbers: Vec<u8>,
}

impl Problem {
    fn new() -> Self {
        return Problem {
            numbers: vec![
                60, 90, 108, 120, 135, 140, 144, 150, 156, 160, 162, 165, 168, 170, 171, 172, 174,
                175, 176, 177, 178, 179,
            ],
        };
    }

    fn solve(&self, a: u8) -> bool {
        return self.numbers.contains(&a);
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u8>() {
            let ans = self.solve(input.scalar());
            match ans {
                true => writeln!(stdout, "YES").unwrap(),
                false => writeln!(stdout, "NO").unwrap(),
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
{% for stdin, stdout in codeforces.test_cases %}
    #[test]
    fn case_{{ loop.index }}() {
        assert_eq!(crate::Problem::_test("{{ repr(stdin)[1:-1] }}"), "{{ repr(stdout)[1:-1] }}");
    }
{% endfor %}
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
