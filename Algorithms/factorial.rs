fn main(){
    println!("Factorial of 20 is {}", factorial(20));
    println!("Factorial of 20 is {}", rec_factorial(20));
}

// Iterative factorial function
fn factorial(mut number : u64) -> u64 {
    let mut fact : u64 = 1;
    
    while number > 0 {
        fact *= number;
        number -= 1;
    }
    fact
}

// Iterative factorial function
fn rec_factorial(number : u64) -> u64 {
    if number > 0 {
        return number * rec_factorial(number - 1);
    }
    return 1;
}
