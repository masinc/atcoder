use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n],
        r: [usize; n],
    }
    let sum = (b.iter().sum::<usize>() + r.iter().sum::<usize>()) as f64;
    let res = sum / (n as f64);
    println!("{}", res);
}
