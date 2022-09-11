use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cur = n;
    let mut primes = vec![];

    for i in 2..=n {
        if i * i > n {
            break;
        }
        loop {
            if cur % i != 0 {
                break;
            }
            primes.push(i);
            cur = cur / i;
        }
    }

    if cur > 1 {
        primes.push(cur);
    }

    println!("{}", primes.into_iter().join(" "))
}
