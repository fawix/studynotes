#[cfg(test)]
mod tests {
    #[test] //<-- indicates that this is a test function
    fn it_works() {
        println!("Hello, Tests!");
        say_hello();

        assert_eq!(2 + 2, 4);
        //asset equality macro checks if two arguments are equal
        //asset inequality macro checks if two arguments are different
        assert_ne!(2 + 2, 5);
        //assert_ne!(2 + 2, 4); //<- fails

        //unlike some languages, in Rust the assert doesn't care 
        //if we swap the direction of our parameters
        assert_eq!(4, 2+2 );

        //these two macros use the PartialEq and Debug traits
        //meaning for custom types we need to implement those in order
        //to run assert_eq / assert_ne (usually we can add derive tag for both of these)
    }

    //it's also possible to have non-test 
    //functions in the test module
    fn say_hello() {
        println!("Oh, hai!");
        //These are generally used to 
        //perform setup steps.
    }

    #[test]
    fn it_fails() {
        //panic!("Nope.");
        //A test fail when something panics.
    }

    use super::*; //to import Rectangle for test below
    //PS: normally this would be on the top 

    #[test]
    fn rec_can_hold() {
        //assert! macro will validate
        //whether something is true (must return bool true)
        assert!(true); // <- like this

        let r1 = Rectangle { length: 10, width: 10 };
        let r2 = Rectangle { length: 5, width: 8 };

        assert!(r1.can_hold(&r2));
        assert!(!r2.can_hold(&r1));
        //assert!(r2.can_hold(&r1)); //<-- fails the test
    }

    #[test]
    fn greet_contains_name() {
        //we can also have custom error messages
        let result = greet("Kevin");
        assert!(result.contains("Kevin"), "Greet did not contain name, value was {}", result);
        //assert!(result.contains("Boohoo"), "Greet did not contain name, value was {}", result);
    }

    #[test]
    #[should_panic] 
    fn will_panic() {
        //we can also check for expected panics
        //the flag above should_panic tells Rust that we 
        //expect this to panic, so we can check how we handle the 
        //expected failure scenarios

        do_not_press(true);
        /*
            This is clearly not a good test
            since it doesn't test a specific panic condition but just whatever
            plus if the panic doesn't happen and test fail the error message is totally unhelpful
        */

    }


    #[test]
    #[should_panic(expected="Red button pressed")]
    fn will_panic_too() {
        //more precise panicking this time...
        //we can use expect to describe what do we expect to fail
        //see above in the should_panic tag
        // that tag defines a sub-string that it expects to see
        //in the panic message to assert whether the panic is the correct one
        do_not_press(true);
    }

    /*
    
        note that Rust will run the tests in parallel using different threads
        therefore the tests should not depend on each other or any shared state
        
        The number of threads Rust use for testing can be changed 

            cargo test -- --test-threads=1 
        
        that tells rust to run one test after the other.
        note if we do "cargo test --help" we see the help for cargo test
        if we do "cargo test -- --help" we see the help for the binary running tests

        If we want to see printed values for passing tests (ommited by default) we 
        should add:

            cargo test -- --nocapture

        We can also run a subset of tests by name
        For example if we want to run only 'rec_can_hold' test we can do:
        
            cargo test rec_can_hold

        that specifies a pattern, in case we had passwed 'will_panic'  both the 
        'will_panic'  test and the 'will_panic_too' test would be run
        PS: must make sure tests have proper name to facilitate running sections of code at once
        Also, note that since the module where the tests are becomes part of the name
        we can simply specify the module name to run the tests within the module
            
            cargo test module_name
    */

    //For tests that are time-consuming we can ingore them
    //by default and run only when requested.
    #[test]
    #[ignore]
    fn expensive_test() {
        //long execution code
        println!("...");
    }
    //the test above will not be run unless we ask for it
    //to run ignored tests we must use:
    //  cargo test -- --ignored

    /*
        All of these have been unit tests
        they can share the same file as the code itself ot 
        facilitate ... unit tests can access both 
        public and private APIs

        Integration tests however are a bit special they are 
        required to be on the tests directory at the root of the project
        and can only access the public APIs (since they are meant to do e2e testing)
    */

}


#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn do_not_press(pressed: bool) {
    if pressed {
        panic!("Red button pressed!!!");
    }
}

pub fn add_two(x: i32) -> i32 {
    internal_add(x, 2)
}

fn internal_add (x: i32, y: i32) -> i32 {
    x + y
}