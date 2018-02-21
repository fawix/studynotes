fn main() {
    println!("Hello, Ownership!");

    /*
        Ownership helps Rust make some safety guarantees and is 
        one of Rust's central features.

        Although Rust is not garbage collected, the ownership feature
        allows Rust to free memory that is no longer used without suffernig 
        a runtime penalty.

        Stack
        
        Store values as a LIFO (last in first out) Queue (think of plate pile)
        Note: adding data is refered as push and removing data as pop

        It's really fast because it never has to search the stack, always get from the top of the pile.
        also, all data in the stack must take a known fixed size.
        
        Heap

        The heap is less organized than the stack.
        The heap must be big enough in size to store all the objects,
        when we add an object in the heap, we must tell the OS the size it will occupy then
        the OS will find an empty spot somewhere in the Heap and add to it, 
        and return a reference pointer to the location. This process is known as allocating [on the heap].
        (think of being seated in a restaurant)

        Note: pushing values into a stack is not considered allocating, because the stack has a known fixed size objects.

        For example: pointers are a known fixed size so we can store then on the stack, but when we want the actual data,
        we have to follow the refence to the heap.

        Generally the heap is slower, since we need to allocate the space and follow a pointer to the data.


        Onwership:
        Keeping track of what parts of code are using what data on the heap and minimizing the ammount of duplicate data on the heap
        and cleaning up unused data we dont need so we dont run out of heap space are problems that ownership addresses.

        Ownership Rules:

        1. Each value in Rust has a variable that is called its owner.
        2. There can only be one owner at a time
        3. When the owner goes out of scope, the value will be dropped.

        Note: the scope here works like the scope in C/C++.

        Example:        
    */

    { // new scope

        // s not valid above this point    
        let s = "oh, hai!"; // s valid below this point
        println!("{}", s)

    } // s not valid below this point

    // println!("{}", s) // throws exception

    /*
    String types can be of unknown size and as such must be allocated on the heap.
    (think about user input)

    This means:

    1. The memory must be requested from the OS at runtime
    2. We need a way of returning this memory to the OS when we're done

    See the following example:
    
    */

    let mut s = String::from("oh, hai!");

    //observe that this type of string can be mutated (due its nature)
    //note: literals cannot be mutated
    s.push_str(" Heap party!");
    println!("{}", s);

    /*
        In case of string literal, the text is hardcoded directly into the final executable.
        Making it very fast (due imutability).

        When we call String::from we take care of item #1 above (requesting memory at runtime)
        and effectively alloacting it on the heap.

        However for item #2, Rust doesn't have a GC (garbage collector) that keeps track of this; 
        however unlike C the program is also not required to indentify and free memory once its no 
        longer being used. In Rust, the memory is automatically returned when the variable goes 
        out of scope.

        When the variable goes out of scope, to free memory a special function called "drop" is invoked by Rust.
        //Looks like RAII patterns in C++

        let's see how this affects us
    */

    // Moving Values

    let x = 5; // bind value 5 to x
    let y = x; // copy value in x and bind to y
    //because integers are known fixed size
    //x and y are pushed into the stack.

    let s1 = String::from("hello");
    println!("s1: {}", s1);
    let s2 = s1;
    println!("s2: {}", s2);
    //println!("s1: {}, s2: {}", s1, s2); // <-- causes a failure because s1 is no longer valid.
    /*
        because the way string works this operation 
        is not the same as the above. A string consists of 3 parts
        a pointer, a length and a capacity
        the pointer is a refence to the memory that is storing the value
        the length is how much memory in bytes the value is using
        the capacity is the total amount of memory in bytes the String has received from the OS.
        This structured is stored on the stack. 
        The memory that holds the actual data/value is allocated on the heap.
        when we assign s2 to s1 we copy on the contents in the stack 
        meaning the pointer, the length and the capacity. The data on the heap is left untouched.
        however since both are pointing to the same heap we need to ensure memory safely, specially when freeing memory.

        To ensure memory safety Rust will consider s1 to be no longer valid and therefore doesn't need to do anything else
        (meaning no memory is freed after the let s1 line.)

        In rust this is known as a Move (like shallow copy in C++ but it also invalidates previous ref -- which C++ does not).
        This means that Rust will never create deep copies of the data automatically, therefore any automatic copy can be assumed to use Move.
        This Move operation is inexpensive in terms of runtime performance.
             
    */


    // Cloning Values

    let s1 = String::from("hellow");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    /*
        The above will work just fine, and this is a "deep copy"
        This will copy the data from the heap as well and create completly 
        separated references. 
        
        A clone call may be expensive, take it as a visual indicator that 
        something different is going on.
    */


    // Copy (for stack only)

    // remember x, y above? they are a stack only 
    // meaning x is still valid.
    println!("x: {}, y: {}", x, y);
    /*
        for values that live entirely in the stack there is no
        difference between deep and shallow copy.

        However even for values that live only on the stack we can 
        add a "Copy" trait, which causes the old value to be usable 
        after a copy (like the above).

        Rust will not let us annotate a type with Copy in case it implements Drop.

        As a general rule:
        1. scalars can be Copy
        2. Values that requires allocation cannot be Copy

        Here are some values that are copy:
        - integer types
        - boolean types
        - floating point types
        - tuples if they contain only copy types; e.g. (i32, i32)
    */

    // Ownership and Functions

    /*
        The semantics of passing a value to a function is similar to assigning a value 
        to a variable.

        Passing a variable to a functino will move or copy, just like assignment.
    */

    let s = String::from("ownership and functions example"); // s comes into scope

    takes_ownership(s); // s value moves into the function ...
    // .. and so is no longer valid here
    //println!("{}", s) // will fail

    let s1 = takes_ownership(); // fn moves the return value into s1
    let s2 = String:from("O.o");
    let s3 = takes_and_gives_back(s2); // s2 moved into fn scope
    //and moves the return to s3

    let x = 5; // x comes into scope

    makes_copy(x); //x would move into function...
    //... but i32 is Copy so its still ok here
    println!("main x: {}", x);

} // here x goes out of scope, s1 & s3 goes out of scope and is dropped, s & s2 goes out of scope but it was moved so nothing happens

fn takes_ownership (some_string: String) { // some_string goes into scope
    println!(" take_ownership: {}", some_string);
}// here some_string goes out of scope and `drop` is called so the memory is freed

fn makes_copy(some_number: i32 ) { // some_number goes into scope
    println!("makes_copy: {}", some_number);
}// some_number goes out of scope. Nothing special happens

fn takes_and_gives_back(some_string: String) -> String {// some_string goes into scope
    some_string // some_string is returned and moves back into the calling function
}

fn gives_ownership() -> String {
    //will move its return value into the calling function
    let some_string = String::from("hai");
    some_string // is returned and moves out of scope
}

/* 
    The ownership of a variable follows the same pattern every time: 
    assigning a value to another variable moves it. 
    
    When a variable that includes data on the heap goes out of scope, 
    the value will be cleaned up by drop unless the data has been moved 
    to be owned by another variable.
*/
