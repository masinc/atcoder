use im_rc::HashMap;
use proconio::input;

fn calc_core(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }

    if let Some(x) = memo.get(&n) {
        *x
    } else {
        let r = calc_core(n - 1, memo) + calc_core(n - 2, memo);
        memo.insert(n, r);
        r
    }
}

fn calc(n: usize) -> usize {
    calc_core(n, &mut HashMap::new())
}

fn main() {
    input! {
        n: usize,
    }

    println!("{}", calc(n))
}
