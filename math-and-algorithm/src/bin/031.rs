use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let a = {
        let mut aa = Vec::with_capacity(n + 1);
        aa.push(0);
        aa.extend(a.into_iter());
        aa
    };

    let mut memo = vec![0usize; n];
    memo[1] = a[1];

    for day in 0..(n - 2) {
        let today = memo[day];

        memo[day + 2] = usize::max(memo[day + 2], today + a[day + 2]);

        if day + 3 < n {
            memo[day + 3] = usize::max(memo[day + 3], today + a[day + 3]);
        }
    }

    let r = &memo[(n - 1)..n].iter().max().unwrap();
    println!("{}", r)
}
