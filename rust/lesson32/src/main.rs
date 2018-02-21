fn main() {
    println!("Hello, Smart Pointers!");

/*
    These are generally a struct.. such as String and Vec<T>.
    The difference between struct and smart pointer is that the 
    smart pointers implement the Deref and Drop traits.

    Deref -> allows an instance of the smart pointer struct to behave like
    a reference, so that we can write code that works with references or smart pointers

    Drop -> allows to customize the code that gets run when an instance of the smart pointer goes
    out of the scope.

    Smart Pointer: Box<T> 

    Generally used for allocating values in the heap.
    Boxes don't have performance overhead other than being on the heap.
    They are commonly used in the following situation:
        - When we have a type whose size can't be known at compile time
          and we want to use the size value.
        - When we have a large amount of data that we need to transfer ownership
          but ensure the data won't be copied when that happens (avoid penalty of copy)
        - When we only care if the type implements an specific trait rather than a concrete type

*/

    //Create a new box to store an i32 in the heap:
    let b = Box::new(5); //when Box goes out of scope it's deallocated.
    println!("b = {}", b);

    //This enable recursive types to exist.. like Cons List.
    //The following wouldn't compile
    //enum List {
    //    Cons(i32, List),
    //    Nil
    //}
    //That is because it trys to make a recursive type directly *see List in Cons*
    //For that to work we put the list in a Box so we only store it's reference in the stack, the rest 
    //goes into the heap.
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil
    }
    //that box enables Rust to compute how much space it needs.
    //use List::{Cons, Nil}; //Normally we have this outside main so we can just add the 'use' statement
    //and simplify the line below by removing all the 'List::' 
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list);

    //Note that when Box goes out of the scope, all heap data is cleaned up as well,
    //because that is how Drop is implemented for Box.


/*

    Smart Pointer: Rc<T>

    Reference counted type that enables multiple ownerships

    Smart Pointers: Ref<T> & RefMut<T>

    enforces borrowing rules at runtime instead of compile time, 
    they can be acessed through RefCell<T>






*/
}
