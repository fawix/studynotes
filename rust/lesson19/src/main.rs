/*
    Now that we saw how to create libraies
    let's see how to use them
*/

// the use statement allows us 
//to shorten function calls
use a::series::of;
use a::series::of::nested_modules;

use TrafficLight::{Red, Yellow};

use TrafficLight1::*;

fn main() {
    println!("Hello, use!");

    
    //To call a module that is nested within modules we do like this
    //use te namespace operator :: 
    a::series::of::nested_modules(); //this is how we call without the use statement

    of::nested_modules(); // this only works because of 'use a::series::of;' statement above
    //we can refer to the of module and Rust will know to 
    // use a::series prefix namespace due the use statement above
    // this will bring the module 'of'  into our current scope, it does not bring the children 

    nested_modules(); // this only works because of 'use a::series::of::nested_modules;' statement above
    //this will bring only the function into our current scope

    //This also applies to enums (since they form a namespace) ... see use statement above
    //TrafficLight is our enum

    let r = Red;
    let y = Yellow;
    let g = TrafficLight::Green; //not imported by use statement

    //we can use * as a wildcard to import all functions under the namespace:
    let r1 = Red1;
    let y1 = Yellow1;
    let g1 = Green1;
    
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules () {
                println!("nested!");
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}


enum TrafficLight1 {
    Red1,
    Yellow1,
    Green1
}

#[cfg(test)]
mod tests {
    //you can also invoke use inside modules:
    use super::a::series::of;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        of::nested_modules();
    }
}