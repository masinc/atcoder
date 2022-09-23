use itertools::Itertools;
use proconio::input;
use std::mem::swap;

#[inline]
fn max_min(max: &mut usize, min: &mut usize) {
    if min > max {
        swap(max, min);
    }
}

fn main() {
    input! {
        n: usize,
        a: [isize; n - 1],
        m: usize,
        b: [usize; m]
    }

    let mut memo: Vec<isize> = vec![0; n + 1];

    for (mut lx, mut rx) in b.into_iter().tuple_windows() {
        max_min(&mut rx, &mut lx);

        memo[lx] += 1;
        memo[rx] -= 1;
    }

    let r = (&memo[1..n])
        .iter()
        .scan(0, |acc, x| {
            *acc += *x;
            Some(*acc)
        })
        .enumerate()
        .map(|(i, x)| x * a[i])
        .sum::<isize>();

    println!("{:?}", r);
}
