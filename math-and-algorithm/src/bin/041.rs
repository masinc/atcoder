use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        v: [(usize, usize); n]
    }

    let mut memo = vec![0; t + 1];
    for (l, r) in v {
        memo[l] += 1;
        memo[r] -= 1;
    }
    memo.pop();

    let r = memo
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .map(|x| x.to_string())
        .collect_vec();

    println!("{}", r.join("\n"))
}
