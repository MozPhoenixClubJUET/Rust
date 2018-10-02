fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 2;

        if count == 4 {
            println!("four");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

       if count == 8 {
            println!("OK, that's enough eight");
 
            // Exit this loop
            break;
        }
    }
}
