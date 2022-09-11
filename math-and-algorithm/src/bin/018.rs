// use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let r = [[100, 400], [200, 300]]
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| a.iter().filter(|x| x == &y).count())
                .product::<usize>()
        })
        .sum::<usize>();

    println!("{}", r);
}
