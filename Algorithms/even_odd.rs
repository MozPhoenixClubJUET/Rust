
use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn main() {
       println!("\n");
       println!("\tOdd and Even Number Checker in Rust");
       println!("\n");

    let value: i32 = grab_input("Enter a Number ")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));

   if value % 2 == 0 {
    println!("\n"); 
    println!("The number {} is an EVEN number.",value);
} else {
    println!("\n");
    println!("The number {} is an ODD number.",value);
   }

    println!("\n"); 
    println!("End of Program");
}

fn grab_input(msg: &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}: ", msg);
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn exit_err<T: Display>(msg: T, code: i32) -> ! {
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)

}
