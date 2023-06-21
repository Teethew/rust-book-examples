fn main() {
    //constantes
    const FIVE: i32 = 5;

    // variavel (imutavel)
    let x = FIVE;

    // shadowing permanente de uma variavel
    let x = x + 1;

    {
        // shadowing por escopo de uma variavel
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing como forma de mudar tipo e valor de variavel imutavel
    let spaces = "   ";
    let spaces = spaces.len();
}
