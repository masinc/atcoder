use itertools::Itertools;
use proconio::input;

#[inline]
fn bits(n: u64) -> Vec<usize> {
    let s = format!("{:#064b}", n);

    s.chars()
        .skip(2)
        .map(|c| c.to_string().parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

fn calc(x: usize, indexes: &[usize], r: &mut Vec<usize>) {
    if x == 0 {
        return;
    }

    r.extend(indexes.iter().combinations(x).map(|x| {
        x.into_iter()
            .fold(0, |acc, i| acc + (2u64.pow(*i as u32)) as usize)
    }));

    calc(x - 1, indexes, r);
}

fn main() {
    input! {
        n: u64,
    }

    let nb = bits(n);

    let indexes = nb
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, x)| x == &&1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut r = vec![0];
    calc(indexes.len(), &indexes, &mut r);
    r.sort();

    let r = r
        .into_iter()
        .map(|x| x.to_string())
        .collect_vec()
        .join("\n");
    println!("{}", r);
}
