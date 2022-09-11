use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let count = (1..=n)
        .into_iter()
        .filter(|a| a % x == 0 || a % y == 0)
        .count();

    println!("{}", count)
}
