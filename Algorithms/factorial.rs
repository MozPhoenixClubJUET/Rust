/// Iterative factorial function
fn factorial(mut number : u64) -> u64 {
    let mut fact : u64 = 1;
    
    while number > 0 {
        fact *= number;
        number -= 1;
    }
    fact
}

/// Iterative factorial function
fn rec_factorial(number : u64) -> u64 {
    if number > 0 {
        return number * rec_factorial(number - 1);
    }
    return 1;
}

#[test]
fn test_factorials(){
    asserteq!(2432902008176640000, factorial(20));
     asserteq!(2432902008176640000, rec_factorial(20));
}
