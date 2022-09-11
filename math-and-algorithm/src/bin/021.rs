use proconio::input;

#[inline]
fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[inline]
fn combination(n: usize, r: usize) -> usize {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    input! {
        n: usize,
        r: usize,
    }

    println!("{}", combination(n, r));
}
