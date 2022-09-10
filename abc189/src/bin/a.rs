use std::collections::HashSet;
use text_io::read;

fn main() {
    let s: String = read!();

    let s: HashSet<_> =s.chars().collect();
    let r = if s.len() == 1 { "Won" } else { "Lost" };
    println!("{}", r);
}
