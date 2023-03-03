use std::io;

use rand::Rng;

fn main() -> () {
    print!("Guess the number! - ");
    println!("Please input your guess");

    let mut guess = String::new();

    // gera número aleatório entre 1 e 100, após recuperar um RNG com seed inicial dada pelo sistema
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    /*
    io::stdin() é uma função que permite usar o stdin,
    read_line() é uma função que guarda o valor da linha lida em uma variável mutável, através do borrow checker e retorna um Result
    expect() é executada caso o Result retornado por read_line() seja diferente de Ok
    */

    //exemplo não idiomático
    let result = io::stdin().read_line(&mut guess);

    if result.is_ok() {
        println!("You guessed it: {guess}");
    }

    //exemplo idiomático
    io::stdin()
        .read_line(&mut guess)
        .expect("should have a value");

    println!("You guessed it: {guess}")
}
