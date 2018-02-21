/*
 * Just to go over some of the types and 
 * structures available in Rust.
 */

fn main() {

    println!("Hello, world!");

    let x = 5; // regular immutable variable
    println!("x = {}", x);

    //x = 6 // <-- this line causes a compiler error | Wheee!
    //println!("x = {}", x);

    let mut y = 5; //mutable variable (due mut keyword)
    //mut declares the intent to mutate the value in the future
    println!("y = {}", y);

    y = 6; // <-- this is fine
    println!("y = {}", y);

    const MY_CONST_VALUE: u32 = 100_000; //<-- constant
    //The diff between immutable variable and constant is
    //- not allowed to use mut with constant
    //- the type of the value must be annotated
    //- Can be declared in any scope, including global scope
    //- May only be set to a constant expression.
    //- Constants are valid within the declared scope all the time
    println!("MY_CONST_VALUE = {}", MY_CONST_VALUE);

    let x = x * 2; // <-- this is shadowing
    println!("x = {}", x);
    // we are declaring a new variable called X
    // based on the previous value of X.
    // Essentially, the initial X is shadowed by the new X.
    // Also because we are affectively creating a new variable
    // we can also change the type of the variable

    // Rust is a statically typed language...
    // Which means it must know the type of the variables
    // at compile time.
    //
    // Note however that the compiler can usually infer the 
    // type we want to use, as seen above. However at points we 
    // might need to annotate the variable (for example when invoking string parse())
    //
    // Here are the common data types supported:
    
    //Scalar Types (Integers)
    
    let x: i8 = 5; //Signed 8-bit number
    println!("x = {}", x);

    let x: u8 = 5; //Unsigned 8-bit number
    println!("x = {}", x);

    let x: i16 = 5; //Signed 16-bit number
    println!("x = {}", x);

    let x: u16 = 5; //Unsigned 16-bit number
    println!("x = {}", x);

    let x: i32 = 5; //Signed 32-bit number | Rust default
    println!("x = {}", x);

    let x: u32 = 5; //Unsigned 32-bit number
    println!("x = {}", x);

    let x: i64 = 5; //Signed 64-bit number
    println!("x = {}", x);

    let x: u64 = 5; //Unsigned 64-bit number
    println!("x = {}", x);

    let x: isize = 5; //Signed size number
    println!("x = {}", x);

    let x: usize = 5; //Unsigned usize number
    println!("x = {}", x);

    let x = 5; //Signed 32-bit number | Rust default
    println!("x = {}", x);
    //signed and unsigned works exactly the same as in C/C++
    //signed numbers are stored using "two complement" representation
    //each signed variant can store numbers from -2^(n-1) to 2^(n-1) - 1
    //meaning for 8-bit for example it can store from -128 to 127
    //while signed can store from 0 to 2^n - 1 (meaning from 0 to 255)
    //
    //The isize and usize are platform dependant,
    //If it's a 64-bit architecture it will be 64-bits
    //on a 32-bit architecture it will be 32-bits and so on.
    //
    //Note:
    //Primary use-case for isize and usize is indexing collections
    //For now.. tons of interesting things here :)
    
    //Floating-Point Types
    
    let x: f32 = 5.0; //Signed 32-bit floating-point number
    println!("x = {}", x);

    let x: f64 = 5.3; //Signed 64-bit floating-point number | Rust default
    println!("x = {}", x);

    let x = 5.3; //Signed 64-bit floating-point number | Rust default
    println!("x = {}", x);

    //Since we're talking about numbers ... 
    
    //Numeric Operations

    let sum = 10 + 5; //addition
    println!("sum = {}", sum);

    let difference = 10 - 5; //subtraction
    println!("difference = {}", difference);

    let product = 10 * 5; //multiplication
    println!("product = {}", product);

    let quotient = 10 / 5; //division
    println!("quotient = {}", quotient);

    let remainder = 10 % 5; //module
    println!("remainder  = {}", remainder);


    //Boolean Type

    let t = true;
    println!("t = {}", t);

    let f: bool = false;
    println!("f = {}", f);


    //Character Type
    
    let c = 'Z';
    println!("c = {}", c);
    let z = 'ℤ';
    println!("z = {}", z );
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat  = {}", heart_eyed_cat );

    //Rust's char type represents unicode scalar value, which means it can represetn a lot more
    //than just ASCII. Accented letters, Chinese/Japanese/Korean ideographs, emoji and zero-width
    //spaces are all valid char types.
    
    // Compound Types
    // Can group multiple values together such as arrays and tuples
    
    //Tuples
    // It's a general way of grouping together values with a variety of types 
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //println!("tup = {}", tup);// <-- will throw exception at this point
    //looks like we need to implement a fmt display :)

    let (x, y, z) = tup; //very similar to the way scala handles this
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    let tup = (500, 6.4, 1, 'Z', "String", false);
    let (x, y, z, w, l, k) = tup; //very similar to the way scala handles this
    //including the fact that this is using pattern matching! This is called "destructuring" 
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("w = {}", w);
    println!("l = {}", l);
    println!("k = {}", k);

    //Note that we can also access tuple elements via '.'
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);
    println!("tup.3 = {}", tup.3);
    println!("tup.4 = {}", tup.4);
    println!("tup.5 = {}", tup.5);

    //we can also assing specific position to variables...
    let x = tup.0;
    println!("x = {}", x);


    //Arrays 

    //it's a group of values of same type
    //In Rust arrays are also immutable,

    let a = [1, 2, 3, 4, 5];
    println!("a[0] = {}", a[0]);
    //Also, Rust's array is allocated on the stack not on the heap.
    //Note, vector is similar to array however it's allowed to
    //grow and shrink... it's also part of the standard library.


    //println!("a[10] = {}", a[10]); // <-- index out of bounds exception at runtime.
    //Note that the in this scenario Rust will not allow access to the memory
    //it will instead crash with the Error (this is more inline to Java)


}
