use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    }
    let mut v = HashSet::new();
    v.insert(0);

    for a in a {
        v.extend(v.iter().map(|x| x + a).collect::<Vec<_>>());
    }

    let r = v.contains(&s);
    println!("{}", if r { "Yes" } else { "No" })
}
