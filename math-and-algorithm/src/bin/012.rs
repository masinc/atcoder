use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut is_prime = true;
    for i in 2..=(n as f64).sqrt().ceil() as usize {
        if n % i == 0 {
            is_prime = false;
            break;
        }
    }

    println!("{}", if is_prime { "Yes" } else { "No" })
}
