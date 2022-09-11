use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize
    }

    let mut count = 0;

    for x in 1..=n {
        for y in 1..=n {
            if x + y <= s {
                count += 1;
            }
        }
    }

    println!("{}", count)
}
