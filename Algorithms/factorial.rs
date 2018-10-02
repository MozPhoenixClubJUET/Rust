fn main(){
    let mut number = 20;
    println!("Factorial of 20 is {}", factorial(&mut number));
}


// function in Rust To Print Factorial of a number 

fn factorial(number : &mut u128) -> u128 {
    let mut fact : u128 = 1;
    while number > &mut 0 {
        fact*= *number;
        *number = *number -1;
    }
    fact
}