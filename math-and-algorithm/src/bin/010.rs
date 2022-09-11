use proconio::input;

#[inline]
fn factorial(n: usize) -> usize {
    (1..=n).into_iter().fold(1, |acc, x| acc * x)
}

fn main() {
    input! {
        n: usize
    }

    println!("{}", factorial(n))
}
