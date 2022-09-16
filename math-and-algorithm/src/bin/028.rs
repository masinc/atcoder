use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n]
    }

    let mut dp = Vec::with_capacity(n);
    dp.push(0);
    dp.push(isize::abs(h[1] - h[0]));

    for i in 2..n {
        let p1 = isize::abs(h[i - 1] - h[i]) + dp[i - 1];
        let p2 = isize::abs(h[i - 2] - h[i]) + dp[i - 2];

        dp.push(isize::min(p1, p2));
    }
    println!("{}", dp.last().unwrap())
}
