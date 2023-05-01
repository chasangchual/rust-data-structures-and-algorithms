pub fn print_multiplication_table() {
    let lh:i8 = 1;
    let rh:i8 = 1;

    for lh in 1..=9 {
        for rh in 1..=9 {
            let value = lh * rh;
            println!("{} * {} = {}", lh, rh, value);
        }
    }
}