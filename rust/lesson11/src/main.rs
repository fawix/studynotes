fn main() {
    println!("Hello, slices!");
    /*
        Slice is a datatype that does not have ownership.
        Slices let you reference a contiguous sequence of elements
        in a collection of elements (rather than the whole collection)
    */

    //A string slice is a reference to part of a string
    //Looks like the following:
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];//range syntax lower bound inclusive upper bound eclusive
    //this is a reference to a portion of the string

    /*
        The slice data structure stores the start position in a pointer
        and the length of the slice,  
        for the range syntax we can omit the zero as a shorthand:
        [0..5] can be written as [..5]
        likewise if we wish to include the string to the last element/byte
        we can drop the last value ... for example
        [6..len] can be written as [6..]

        slice type is &str 
    */

    println!("s = {}, hello = {}, world = {}", s, hello, world);

    let s1 = String::from("slices are fun");

    println!("{}", first_word_slice(&s1));

    /*
        Now that we know about slices let's revist literals

        let s = "Hello, World!";
        the type of s here is &str - it's a slice pointing to that specific point of the binary.
        this is also why string literals are immutable; &str is an immutable reference.

        knowing this we can improve the first_word_slice method above.
    */

    let literal = "hello world 1";
    let not_literal = String::from("world hello 2");

    println!("{}", improved_first_word_slice(literal));
    println!("{}", improved_first_word_slice(&not_literal[..]));

    /*
        Other types of slices (non-string)

    */

    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3]; //the type is &[i32]
    //it works the same way as string slices... by storing references and length.
    
    for item in slice {
        println!("{}", item);
    }

    /*
        The concepts of ownership, borrowing and slices are what ensures memory safety in Rust
        programs at compile time. Rust language still gives you control over your memory usage like other systems programming
        languages, but haing the owner of data automatically clean up that data means more abstraction.
    */
}

/*
    the below is a poor attempt at illustrating the problem
    when we don't have slice :) 
*/
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

//Making it take a slice as input makes it more general
//In case we need to pass a string we can always give it the slice of the full string [..]
fn improved_first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
//Not sure how much I agree with this yet...
