use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(f64, f64); n]
    }

    let r: f64 = a.into_iter().map(|(p, q)| q / p).sum();

    println!("{:.12}", r);
}
