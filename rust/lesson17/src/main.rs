fn main() {
    println!("Hello, Option!");

    //Option is a enum in the standard library
    //(the book says "very useful enum")

    /*
        Option expresses the scenario where a variable can
        be either something or nothing.

        Expressing this concept in terms of the type system means the compiler can 
        check if we handled all the cases we should have.

        Rust doesn't have null (!!)
        //Now, that is different!

        Therefore Option encodes the concept of present or absent.
        Option is defined as follows:

        enum Option<T> {
            Some(T),
            None
        }

        Option is included in the prelude so there is no need to import it.
        So are the variants (meaning we can use Some and None without the prefix).
    */

    let some_number = Some(5);
    let some_string = Some("string"); // Compiler can infer type.
    let absent_number: Option<i32> = None; //When using none we need to tell Rust the type of option.

    // Cool! This actually forces an entire new paradigm in programing
    // the programmer must think of all the use cases beforehand 
    // (although that should be the norm by experience nowadays that is the exception)

    /*
        Note that the Option<T> is different than T
    */

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    println!("x={} y={:?}", x, y);

    //let sum = x + y; // Throws an error (trait bound not satisfied)
    //Means that Rust doesn't know how to add i8 to Option<i8>
    //to use this we would need to use pattern matching!
    //see plus_one below

    println!("y+1={:?}",plus_one(y));
    println!("2+1={:?}",plus_one(Some(2)));
    println!("None+1={:?}",plus_one(None));

    // if let pattern matching with Option:
    let some_u8_value = Some(3u8);
    if let Some(3u8) = some_u8_value {
        println!("Some Trees ☘");
    }

}

fn plus_one (x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
