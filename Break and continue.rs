fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 2;

        if count == 6 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
