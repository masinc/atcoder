use proconio::input;
use std::collections::VecDeque;

#[inline]
fn max_min(max: &mut usize, min: &mut usize) {
    let tmp_max = usize::max(*max, *min);
    let tmp_min = usize::min(*max, *min);

    *max = tmp_max;
    *min = tmp_min;
}

#[inline]
fn get_greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;
    max_min(&mut max, &mut min);

    while max > 0 && min > 0 {
        max %= min;

        if max == 0 {
            break;
        }

        max_min(&mut max, &mut min);
    }

    usize::max(max, min)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut a: VecDeque<_> = a.into_iter().collect();

    let mut x = a.pop_front().unwrap();
    let mut y = ::core::usize::MAX;

    while !a.is_empty() {
        if usize::max(x, y) == x {
            x = a.pop_front().unwrap();
        } else {
            y = a.pop_front().unwrap();
        }

        x = get_greatest_common_divisor(x, y);
        y = ::core::usize::MAX;
    }

    println!("{}", usize::min(x, y))
}
