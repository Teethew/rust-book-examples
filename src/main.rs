/*
You made it! 
This was a sizable chapter: 
you learned about variables, scalar and compound data types, functions, comments, if expressions, and loops!

To practice with the concepts discussed in this chapter, try building programs to do the following:

-[x] Convert temperatures between Fahrenheit and Celsius.

-[ ] Generate the nth Fibonacci number.

-[ ] Print the lyrics to the Christmas carol “The Twelve Days of Christmas”,
taking advantage of the repetition in the song. */

fn main() {
    println!("{} °C | {} °F", 14.0, celsius_to_fahrenheit(14.0));
    println!("{} °C | {} °F", fahrenheit_to_celsius(57.2), 57.2);
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * (9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/9.0)
}