use proconio::input;

#[inline]
fn contains(a: &[usize], x: usize) -> bool {
    let len = a.len();
    let mid = len / 2;

    if a[mid] == x {
        true
    } else if len == 1 {
        false
    } else if a[mid] < x {
        contains(&a[mid..], x)
    } else if a[mid] > x {
        contains(&a[..mid], x)
    } else {
        unreachable!();
    }
}

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n]
    }

    a.sort();
    let r = if contains(&a, x) { "Yes" } else { "No" };
    println!("{}", r);
}
