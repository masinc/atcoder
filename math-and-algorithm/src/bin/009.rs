use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    }
    let mut v = FxHashSet::default();
    v.insert(0);

    for a in a {
        v.extend(v.iter().map(|x| x + a).collect::<Vec<_>>());
    }

    let r = v.contains(&s);
    println!("{}", if r { "Yes" } else { "No" })
}
