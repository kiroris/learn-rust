use std::cmp::Ordering; // enumeration with options: Less, Greater and Equal
use rand::Rng; //  set range to a random number(get_range)
use std::io;   // input/output


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()   // generator of random number
                        .gen_range(1..=100); // sets range to a random number

    loop {   
        println!("Please input your guess.");

        let mut guess = String::new(); // new empty "String"

        io::stdin()
            .read_line(&mut guess) //input 
            .expect("Failed to read line"); //error handler
            
        let guess: u32 = match guess.trim().parse() { //we change expect() to match
            Ok(num) => num,
            Err(_) => {
                println!("Enter number please!");
                continue;
            }
        };
         
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // trim() in String - removes the space at the beginning and end of the line and delete "\n"
        // parse() in Str - converts a string to another type, which we specify at the beginning (now to a number) {u32}
        // expect - processing Result()

        println!("You guessed: {guess}");

        // match determines which Ordering option was returned
        match guess.cmp(&secret_number) { // cmp compares two values and can be called for anything that can be compared.
            Ordering::Less => println!("Too small!"),  // if {guess} is  than {secret_number}
            Ordering::Greater => println!("Too big!"), // if {guess} is greater than {secret_number}
            Ordering::Equal => {
                println!("You win!");   // if {guess} is  than {secret_number}
                break;
            }
        }
    }
}
