use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    }

    let l1 = (a + b) * (c - d);
    println!("{}", l1);
    println!("Takahashi");
}
