use proconio::input;

fn main() {
    input! {
        n: usize, max: usize,
        xs: [(usize, usize) ;n]
    };

    let max = max * 100;
    let mut cur: usize = 0;
    for (i, x) in xs.into_iter()
        .map(|(v, p)| { v * p }).enumerate()
    {
        cur += x;

        if cur > max {
            println!("{}", i+1);
            return;
        }
    }

    println!("-1");
}
