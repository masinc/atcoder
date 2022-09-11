use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
        b: [f64; n]
    }

    let r = a
        .iter()
        .zip(&b)
        .map(|(ax, bx)| (ax / 3.0) + (bx * 2.0 / 3.0))
        .sum::<f64>();

    println!("{:.12}", r);
}
