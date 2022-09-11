use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let r = [1, 2, 3]
        .iter()
        .map(|color| a.iter().filter(|x| x == &color).count())
        .map(|count| count * (count - 1) / 2)
        .sum::<usize>();

    println!("{}", r);
}
