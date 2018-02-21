#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

//impl keyword defines an implementation within the context of 'Rectangle'
impl Rectangle { 
    fn area(&self) -> u32 { //this is a method .. much like method definition of Go (not real OOP method).
        self.length * self.width
    }

    fn can_contain(&self, other: &Rectangle) -> bool { //multiple params
        self.length >= other.length && self.width >= other.width 
    }

    //We are allowed to define functions without self within the impl block.
    //Those are called associated functions (like String::from)
    //These are often used for constructors

    fn square (size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
     println!("Hello Methods & Associated Functions");
    /*
        methods are similar to functions: 
            1. they are declared using fn keyword
            2. they can have parameters
            3. they contain some code that is run

        methods are different than functions:
            1. they must be defined within the context of a struct (or enum, or trait)
            2. their first parameter is always self (which represents the instance of the struct)

        methods can take ownership of self, borrow it immutabily or borrow it mutably.
        Taking ownership is rare and us used when the method transforms self in something else,
        and wants to prevent the usage of the original version.

    */

    let r = Rectangle { length: 30, width: 50 };
    println!("The area is {} pixels squared", r.area());
    // fun fact: even if the object is a pointer in Rust we still use dot-notation
    // ( in C++ that would use -> )
    // in C++ object->method() is equivalent to (*object).method()
    // In Rust however we can use dot-notation thanks to automatic 
    // referencing and deferencing, calling methods is one of the places
    // where we use this behaviour.
    // That works in Rust because methods have a clear receiver: self
    // Given that and the signature of method, Rust can determine whether
    // to use &self or &mut self, making the borrowing implicit. This facilitates
    // ownership in practice (using methods);

    let r2 = Rectangle { length: 15, width: 25 };

    println!("Can r1 contain r2? {}", r.can_contain(&r2));
    println!("Can r2 contain r1? {}", r2.can_contain(&r));

    //To call an associated function we use the :: notation
    let sq = Rectangle::square(10);
    println!("{:?}", sq);

    //Also note that this function is namespaced by the struct
    // :: notation is used both by namespaces and associated functions
}
