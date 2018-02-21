fn main() {
    println!("Hello, Match!");

    //Match is a flow control operator.
    //This allows for pattern matching (a la Scala).
    //the match is done from top to bottom, meaning 
    //it will execute the first statement that matches.

    //Check Coin enum below.

    println!("Value in cents for Quarter: {} cents", value_in_cents(Coin::Quarter(State::Alaska)));
    println!("Value in cents for Penny: {} cents",Coin::Penny.value_in_cents());

    //Wildcard for pattern matching is like Scala " _ "
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => () // this is similar to scala... or default in other languages
        //like a catch all, if none of the above match use this.
        //change some value to 5u8 and see it match on five
    };

    //We can also do if let combination for pattern matching if we care about one case only...

    let some_u8_value1 = 3u8; 
    match some_u8_value1 {
        3 => println!("Three Trees ☘"),
        _ => ()
    };

    //we can write the above like this instead:
    if let 3u8 = some_u8_value1 {
        println!("Another Three Trees ☘");
    }
    /*
        The general construct follows this structure:
        if let <pattern> = <variable> {
            <code_to_execute>
        }

        it takes a pattern and an expression seperated by an equal ' = '

        This construct is more concise however it deos not povide for exhaustive 
        checking at compile time like match does.
    */
    // we can also combine if let with else block
    //let coin = Coin::Quarter(State::Alabama);
    let coin = Coin::Penny;
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State Quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("count = {}", count);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
    //we can have Quarter hold state info for the design on the back of the coin
    //Let's make State an enum (below)
}

impl Coin {
    
    fn value_in_cents(&self) -> i32 {
        match self {
            Penny => 1,
            Nickel => 5,
            Dime => 10,
            Quarter => 25
        } // notice that pattern matching here also uses => operator!
        //also, no need  to put a ; at the end, neat syntax.
    }
/*
    Note that match must be exhaustive, meaning if we omit 
    one of the possible values the code will not compile.
    Try commenting one of the above and see.
    The error is something like "non-ehaustive patterns"
*/



}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            print!("Lucky Penny! ");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State from quarter is {:?}", state);
            25
        }
    } // notice that pattern matching here also uses => operator!
    //also, no need  to put a ; at the end, neat syntax.
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska
    //etc
}
