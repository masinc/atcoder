use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        v: [(usize, usize); q]
    }

    let b = {
        let mut bb = Vec::with_capacity(n + 1);
        bb.push(0);

        bb.extend(a.iter().scan(0, |state, &x| {
            *state += x;
            Some(*state)
        }));
        bb
    };

    for (x, y) in v {
        println!("{}", b[y] - b[x - 1]);
    }
}
