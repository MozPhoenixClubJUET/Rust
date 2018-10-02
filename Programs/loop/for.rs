fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 2 == 0 {
            println!("even");
        } else {
            println!("{}", n);
        }
    }
}
