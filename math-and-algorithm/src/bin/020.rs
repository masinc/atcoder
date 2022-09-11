use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    for m in (l + 1)..n {
                        if [i, j, k, l, m]
                            .iter()
                            .map(|x| a.get(*x).unwrap())
                            .sum::<usize>()
                            == 1000
                        {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans)
}
