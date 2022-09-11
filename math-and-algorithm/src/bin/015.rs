use proconio::input;

#[inline]
fn max_min(max: &mut usize, min: &mut usize) {
    let tmp_max = usize::max(*max, *min);
    let tmp_min = usize::min(*max, *min);

    *max = tmp_max;
    *min = tmp_min;
}

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    let (mut max, mut min) = (a, b);
    max_min(&mut max, &mut min);

    while max > 0 && min > 0 {
        max = max % min;

        if max == 0 {
            break;
        }

        max_min(&mut max, &mut min);
    }

    println!("{}", usize::max(max, min))
}
