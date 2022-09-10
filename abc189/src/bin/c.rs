use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut max: usize = 0;

    for l in 0..n {
        let mut min = a[l];
        for r in l..n {
            let len = r - l + 1;
            min = usize::min(min, a[r]);
            max = usize::max(max, len * min);
        }
    }
    println!("{}", max);
}
