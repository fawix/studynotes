/*
 * This will be more about functions
 *
 */

fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(100);
    another_function3(100, 3.14);
    another_function3(100);
}


//fn is the keyword for function
fn another_function() {
    println!("another function!");
}

//this one receives a parameter
fn another_function2(x: i32) {
    println!("x = {}", x );
}

fn another_function3(x: i32, y: f64) {
    println!("x = {}", x );
    println!("y = {}", y );
}

//I wonder if we have overload... 
//fn another_function3(x: i32) {
//    println!("x = {}", x );
//}
// .. this one throws a compile error
// ... arrgh... turns out overload in Rust is really ugly.
// There is a sort of thing called Trait ... which is not similar to 
// Scala's trait and then you would need to implement a specific trait to the 
// overlaod and provide different method implementations for each... 
// https://www.reddit.com/r/rust/comments/2umcxv/wait_rust_doesnt_have_function_overloading/
//
// Not. Cool. ... 
//
// Something like the following
//
// trait ArgumentForOverloadableFunction {
//     fn answer(&self) -> i32;
// }
// 
// struct Default;
// struct OneArg(i32);
// struct TwoArgs(i32, i32);
// 
// impl ArgumentForOverloadableFunction for Default {
//      fn answer(&self) -> i32 {
//         0
//      }
// }
// impl ArgumentForOverloadableFunction for OneArg {
//      fn answer(&self) -> i32 {
//         self.0
//      }
// }
// impl ArgumentForOverloadableFunction for TwoArgs {
//     fn answer(&self) -> i32 {
//         self.0 + self.1
//      }
// }
// 
// fn overloadable<T: ArgumentForOverloadableFunction> (arg: T) -> i32 {
//     arg.answer()
// }
// 
// fn main() {
// println!("{}", overloadable(Default));
// println!("{}", overloadable(OneArg(1)));
// println!("{}", overloadable(TwoArgs(2, 3)));
// 
// }
//
// While it is true that this feature can be abused that is also true for any other feature.
// Coming from OOP I learned the value of overloading things just right for beaultifully designed
// APIs ... think about mandatory vs optional parameters. 
//
// Even something like Python's default value on functions would work; but not having the option
// doesn't work. It's a shame because I was liking how well through this language is/was so far. 
//
//
//
//  With that ... moving to check out Go 

//--- post note
// Ok, I went learnt go
// it's a cool language tons of neat features (I specially like defer) but since it don't have overload either 
// I thought I should be fair to Rust keep learning it as well. Not to mention some stuff is better wtihout GC (yep... password managers for example)
// So Rust is a nice addition to my toolbelt anyway
// Might be fun to be able to implement same thing in both to compare lol :)