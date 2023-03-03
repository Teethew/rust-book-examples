use std::io;

fn main() -> () {
    print!("Guess the number! - ");
    println!("Please input your guess");

    let mut guess = String::new();

    /*
    io::stdin() é uma função que permite usar o stdin,
    read_line() é uma função que guarda o valor da linha lida em uma variável mutável, através do borrow checker e retorna um Result
    expect() é executada caso o Result retornado por read_line() seja diferente de Ok
    */

    //exemplo não idiomático
    let result = io::stdin()
        .read_line(&mut guess);

    if result.is_ok() {
        println!("You guessed it: {guess}");
    }

    //exemplo idiomático
    io::stdin()
        .read_line(&mut guess)
        .expect("should have a value");

    println!("You guessed it: {guess}")
    
}
