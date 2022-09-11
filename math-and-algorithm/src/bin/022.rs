use proconio::input;
use rustc_hash::FxHashMap;
use std::hash::BuildHasherDefault;

const MAX: usize = 100_000;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut m: FxHashMap<usize, usize> =
        FxHashMap::with_capacity_and_hasher(n, BuildHasherDefault::default());

    for x in a {
        if let Some(v) = m.get_mut(&x) {
            *v += 1
        } else {
            m.insert(x, 1);
        }
    }

    let m = m;
    let mut ans = 0;

    for i in 1usize..(MAX / 2) {
        let low = m.get(&i).unwrap_or(&0);
        let high = m.get(&(MAX - i)).unwrap_or(&0);

        ans += low * high;
    }

    // 50000
    if let Some(v) = m.get(&(MAX / 2)) {
        ans += (v * (*v - 1)).checked_div(2).unwrap_or_default();
    }

    println!("{}", ans)
}
