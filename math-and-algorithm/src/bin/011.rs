use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut primes = vec![];
    for i in 2..=n {
        let mut is_prime = true;
        for p in primes.iter() {
            if i % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i)
        }
    }

    let r = primes.into_iter().join(" ");
    println!("{}", r)
}
