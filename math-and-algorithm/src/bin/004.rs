use proconio::input;

fn main() {
    input! {
        a: [usize; 3]
    }

    let r: usize = a.into_iter().fold(1, |acc, x| acc * x);
    println!("{}", r);
}
