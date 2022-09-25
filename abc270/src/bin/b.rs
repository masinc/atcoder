use proconio::input;

#[allow(clippy::needless_bool)]
fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize,
    }

    let x_positive = x > 0;
    let y_positive = y > 0;
    let z_positive = z > 0;

    let x_negative = x < 0;
    let y_negative = y < 0;
    let z_negative = z < 0;

    if (x_positive == y_positive) || (x_negative == y_negative) {
        if (x_positive && x < y)
            || (x_negative && x > y)
            || (x_positive == z_positive && x < z)
            || (x_negative == z_negative && x > z)
        {
            println!("-1");
            return;
        }

        if (y_positive == z_positive) || (y_negative == z_negative) {
            if (x_positive && y_positive) || (x_negative == y_negative) {
                println!("{}", x.abs());
            } else {
                println!("{}", x.abs() + (y.abs() * 2));
            }
        } else if (x_positive && y_positive) || (x_negative && y_negative) {
            println!("{}", x.abs() + (y.abs() * 2));
        } else {
            println!("{}", x.abs() + (y.abs() * 2) + (z.abs() * 2));
        }
    } else {
        println!("{}", x.abs());
    }
}
