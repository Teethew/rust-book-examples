fn main() {
    /*
        Tipos de Inteiros (default i32):
      ============================
      ||Length	Signed	Unsigned||
      ============================
      ||8-bit	i8	    u8      ||
      ||16-bit	i16	    u16     ||
      ||32-bit	i32	    u32     ||
      ||64-bit	i64	    u64     ||
      ||128-bit	i128	u128    ||
      ||arch	isize	usize   ||
      ============================
    
        Integer literals:
      ================================
      ||Number literal  Example     ||
      ================================
      ||Decimal	        98_222      ||
      ||Hex	            0xff        ||
      ||Octal	        0o77        ||
      ||Binary	        0b1111_0000 ||
      ||Byte (u8 only)	b'A'        ||
      ================================
    
       Tipos de numeros de ponto-flutuante: 
       f32 e f64 (default). Ambos possuem sinal
    
    */
    
    //operacoes
    
        // addition
        let sum = 5 + 10;
    
        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
    
        // remainder
        let remainder = 43 % 5;
    
        
    // tuplas
    
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
    
        println!("The value of y is: {y}");
    
    }
    