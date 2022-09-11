use proconio::input;

fn main() {
    input! {n: usize}

    let mut v = 0.0;

    for x in (1..=n).map(|x| x as f64) {
        v += (n as f64) / x;
    }

    println!("{:.12}", v)
}
