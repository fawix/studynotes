extern crate rand; // lets rust know we will be using this crate

use std::io;
use std::cmp::Ordering;
use rand::Rng; // this one tells rust to add rand::Rng to scope

fn main() {
    println!("Let me think of a number between 1 and 100 ... \n...\n");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("Secret number is {}", secret_number);
    //thread_rng is seeded by the OS
    //gen_range is inclusive on the lower bound but exclusive
    //on the upper bound, which mean the range 1,101 really specify
    //numbers between 1 and 100.

    println!("Ok! Can you guess it?");

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        // the keyword let is to create a variable
        // in Rust variables are immutable by nature
        // the keyword mut is to tell Rust that this variable
        // is mutable.
        // then we use the String::new() method to assing the value 
        // of a new string to the variable guess.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //read_line returns a "Result" enum
        // that can be Ok or Err
        // If err it will trigger the failure 
        // in expect and will print the message.
        // Something to note:
        // When user presses return key to enter the string value
        // Rust will append a new line \n to the end of the string.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue //similarly to scala  _ is a "catch all wildcard" 
        };
        //Rust have a concept called "shadow"
        //by which we can override the previous value of a variable
        //with a new one. When we say guess: u32 we are effectively
        //creating a new variable of same name of type unsigned 32-bit integer.
        //Note: signed 32-bit integer is i32
        //In this case we are binding guess to the expression guess.trim().parse()
        //so the right hand side refers to the original string value
        //left hand side refers to the new integer value
        //Note on trim:
        //the "trim()" method will remove leading and trailing white spaces from 
        //the variable plus it will remove the trailing newline '\n' char.
        //Note on parse:
        //it can parse a string into some kind of number.
        //because there are several number types available we need to tell Rust 
        //exactly which type to parse to.. that is accomplished by let guess: u32...
        //the colon after guess tells Rust we'll annotate the variable's type.
        //Note:
        //Because we anotate the type of guess to be u32 Rust will infer that 
        //secret_number should also be u32 type!


        println!("You guessed: {}", guess);
        //Yep, string subistitution is {} not %s 
        //quite unusual but worked lol

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal   => {
                println!("Bingo!!");
                break;
            } // , // <-- that comma is interesting.. | note: it still compiles without the last comma
                //I find my last element should have no comma just to be sure :) 
        }// this is very similar to pattern matching in scala!
    }
} //Extra goodies: type cargo open --doc to see documentation including dependancy crates documentation.
