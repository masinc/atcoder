use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        v: [(usize, usize, isize); q]
    };

    let mut memo = vec![0; n + 1];

    for (l, r, x) in v {
        *memo.get_mut(l - 1).unwrap() += x;
        *memo.get_mut(r).unwrap() -= x;
    }

    let mut re: Vec<_> = memo
        .into_iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .tuple_windows()
        .map(|(a, b)| match a.cmp(&b) {
            Ordering::Equal => "=",
            Ordering::Greater => ">",
            Ordering::Less => "<",
        })
        .collect();

    re.pop();
    println!("{}", re.join(""))
}
