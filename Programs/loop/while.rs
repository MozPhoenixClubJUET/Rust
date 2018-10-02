fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 2 == 0 {
            println!("even");
        } 
        else if n % 2!= 0 {
            println!("odd");
        }

        // Increment counter
        n += 1;
    }
}
