use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut r = vec![];

    for i in 1..=(n.sqrt()) {
        if n % i != 0 {
            continue;
        }
        r.push(i);
        if i != (n / i) {
            r.push(n / i)
        }
    }

    r.sort();
    println!("{}", r.into_iter().join("\n"));
}
