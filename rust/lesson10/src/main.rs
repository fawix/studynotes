fn main() {
    println!("References and Borrowing!");

    /*
        Summary:
            1. At any given time you can have either (not both):
                One Mutable Reference
                Multiple immutable References
            2. References must always be valid
    */

    let s1 = String::from("eh, hellow");
    let len = calc_len(&s1);// the ampersand & denotes reference

    // reference allow us to refer to a value without taking ownership of it
    // in this case s (in the function) will point to s1 (in this scope)
    println!("The lenght of '{}' is {}",s1, len);

    // if commented this will fail
    //no_changing(s1);

   let mut s2 = String::from("me, mutable!");
   can_change(&mut s2); // notice how here we pass a mutable reference (important!)
   println!("s2 = {}", s2);

   /* 
    It's important to note that we can only have a single mutable reference per piece of data.
    Meaning the code below will fail to complile:

    let mut s = String::from("mutable!")

    let r1 = &mut s;
    let r2 = &mut s;

    ^
    two mutable references, not allowed.

    This restriction allows for mutation but in a very controlled fashion.
    This allows Rust to prevent data races from conditions such as:
        1. Two or more pointers access the same data at the same time (dirtcow!)
        2. At least one of the pointers is being used to write to the data
        3. There's no mechanism being used to synchronize access to the data

    A similar rules exists for combining mutable and unmutable references like so:

    let mut s = String::from("mutable!")

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // <-- big problem!

    that is because owners of immutable references don't expect the value to change without warning.

    Likewise dangling references are prevented at compile time, 
    let ref_to_nothing = dangle(); //dangle returns a ref to a string

    fn dangle() -> &String {
        let s = String::from("hello"); // new string
        &s //return the reference
    } // s out of scope and gets dropped... buggy code
    //the solution is simple: return the string instead of the reference.

    */

}

fn calc_len(s: &String) -> usize { // notice how we use &String (reference to a String)
    //having a reference as a function parameter is called Borrowing
    s.len() // this is Borrowing
}//s goes out of scope, however since it does not have ownership nothing special happens

//this means we cannot modify the value we borrowed 
//the function below will cause compile failure
//fn no_changing(s: &String) {
//    s.push_str(", it will fail!");
//}

/*
    Just as variables are immutable by default,
    so are references, we're not allowed to modify something
    we have a reference to (that we don't have ownership).
    At least not without being explict about it
*/

fn can_change(s: &mut String) { // notice how this receives a multable reference
    s.push_str(" It will work!");
}
