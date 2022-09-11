use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    }

    let mut r = false;

    for i in 1..n {
        r = a
            .iter()
            .permutations(i)
            .any(|x| x.into_iter().sum::<usize>() == s);
        if r {
            break;
        }
    }

    println!("{}", if r { "Yes" } else { "No" })
}
