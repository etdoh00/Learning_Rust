use std::io;
use std::cmp::Ordering;
use rand::Rng;
//psuedo for making a small guess number game
/*
> Take user input 
> Ensure user input is a number
> Loop guess until the user has guessed correct
> Break and print number when guessed
*/
fn main()
{
    println!("Welcome to the Magic Number, try and guess what it is....");

    let magic_number = rand::thread_rng().gen_range(1..=100); // generates a 32 bit unsigned number in a range 1-100. 

    loop //loop user input
    {
        println!("Enter your guess: ");

        let mut user_guess = String::new(); //user guess is taken as a mutable string, as it will need to change everytime they guess

        io::stdin().read_line(&mut user_guess).expect("Could not read line!"); // reads the user input, crude error handling using expect instead of explictly handling

        //convert user guess to u32 to compare
        let user_guess: u32 = match user_guess.trim().parse() //trims white spaces and new line "\n" after the user presses enter on the line above. THen parse into u32
        {                                                     //this shows Rust's variable shadowing, allowing it to be overwritten without the need of two variables
            Ok(guess) => guess, // match returns 2 enurmerations. Ok meaning its okay and the number is assigned. 
            Err(_) => continue // Err uses the catch all "_" to retrieve anything that is not allowed by "Ok". Continue allows the loop to repeat
        };

        match user_guess.cmp(&magic_number) { // Using the ordering lib crate it can use the arms of the cmp function to look and compare the input and the properties of the "magic number"
            Ordering::Less => println!("Nope, too low. Try again!"),
            Ordering::Greater => println!("Nope, too high. Try again!"),
            Ordering::Equal => { print!("Nice, you got it! The answer was {magic_number}"); break;} // when equal, print statement and break loop
            }
    }
}
