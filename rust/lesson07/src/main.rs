/* Function return values

... In rust that is kinda strange ...

we show the return value with an arrow like the following:
*/

fn five () -> i32 {
    5
} /* notice how we don't need the return instatement  
     that is because the above is an expression (no semicolon) 
     therefore the compiler knows we want to return the expression "5" 
  */

fn plus_one(x: i32) -> i32 {
    x + 1
    //x + 1; // <-- this line will generate an error, which reads type mismatch because it detets an empty
    // return while it expects i32 ... this is due the semicolon which marks the line as an statement instead of expression
}

fn main() {
    println!("Hello, world!");
    let x = five();
    println!("x = {}", x);

    println!("5 plus one: {}", plus_one(5))
}