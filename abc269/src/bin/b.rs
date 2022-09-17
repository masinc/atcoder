use proconio::input;

fn main() {
    input! {
        s: [String; 10]
    }

    let mut a = None;
    let mut b = None;
    let mut c = None;
    let mut d = None;

    for (i, line) in s.into_iter().enumerate() {
        let start = line.find("#");
        if start.is_some() {
            let y = i + 1;
            if a == None {
                a = Some(y);
                c = start.clone().map(|x| x + 1);
                d = line.rfind("#").map(|x| x + 1);
            };

            b = Some(y);
        }
    }

    println!("{} {}", a.unwrap(), b.unwrap());
    println!("{} {}", c.unwrap(), d.unwrap());
}
