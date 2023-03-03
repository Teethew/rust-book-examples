use std::{io::{self, Stdin}, cmp::Ordering, num};

use rand::Rng;

fn main() -> () {
    print!("Guess the number! - ");

    // gera número aleatório entre 1 e 100, após recuperar um RNG da thread atual com seed inicial dada pelo SO
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        /*
        io::stdin() é uma função que permite usar o stdin,
        read_line() é uma função que guarda o valor da linha lida em uma variável mutável, através do borrow checker e retorna um Result
        expect() é executada caso o Result retornado por read_line() seja diferente de Ok
        */

        // exemplo não idiomático
        // let stdin: Stdin = io::stdin();
        // let result: Result<usize, io::Error> = stdin.read_line(&mut guess);

        // if result.is_ok() {
        //     println!("You guessed: {guess}");
        // }
        
        //exemplo idiomático
        io::stdin()
            .read_line(&mut guess)
            .expect("should have an integer value");
    
        println!("You guessed: {guess}");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match é usado de forma similar a um switch case
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
