// https://codeforces.com/problemset/problem/1409/B
// ```python
// from sympy import Max, Min, symbols
//
// a, b, dx, dy, n = symbols('a, b, dx, dy, n', positive=True)
//
// x, y = a-dx, b-dy
// # First subtract from a, then subtract from b
// a0 = Max(a-n, x)
// na = Max(a - a0, 0)
// b0 = Max(b-(n-na), y)
// ab = a0 * b0
// # First subtract from b then subtract from a
// b0 = Max(b-n, y)
// nb = Max(b - b0, 0)
// a0 = Max(a-(n-nb), x)
// ba = a0 * b0
// # comparison
// x, y = symbols('x, y')
// print(Min(ab, ba).subs({dx: a-x, dy: b-y}).simplify())
// ```
//
// ```matlab
// syms a b n positive integer
// syms dx dy
//
// assume(0 <= dx <= a);
// assume(0 <= dy <= b);
//
// x = a - dx;
// y = b - dy;
//
// assumptions
//
// % First subtract from a, then subtract from b
// a0 = feval(symengine, 'max', a-n, x);
// na = feval(symengine, 'max', a-a0, 0);
// b0 = feval(symengine, 'max', b-(n-na), y);
// ab = a0 * b0;
// % First subtract from b then subtract from a
// b0 = feval(symengine, 'max', b-n, y);
// nb = feval(symengine, 'max', b-b0, 0);
// a0 = feval(symengine, 'max', a-(n-nb), x);
// ba = a0 * b0;
// % comparison
// f = feval(symengine, 'min', ab, ba);
// g = subs(simplify(f), {dx, dy}, {a-sym('x'), b-sym('y')});
// simplify(g)
// ```
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

    fn solve(&self, a: i64, b: i64, x: i64, y: i64, n: i64) -> i64 {
        let (anx, bny) = (i64::max(a - n, x), i64::max(b - n, y));
        return i64::min(
            anx * i64::max(b - n + i64::max(a - anx, 0), y),
            bny * i64::max(a - n + i64::max(b - bny, 0), x),
        );
    }

    fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for _ in 0..input.scalar::<u16>() {
            let abxyn: Vec<_> = input.vector(5);
            let ans = self.solve(abxyn[0], abxyn[1], abxyn[2], abxyn[3], abxyn[4]);
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
        assert_eq!(crate::Problem::_test("7\n10 10 8 5 3\n12 8 8 7 2\n12343 43 4543 39 123212\n1000000000 1000000000 1 1 1\n1000000000 1000000000 1 1 1000000000\n10 11 2 1 5\n10 11 9 1 10\n"), "70\n77\n177177\n999999999000000000\n999999999\n55\n10\n");
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    Problem::new().via_io(stdin.lock(), stdout.lock());
}
