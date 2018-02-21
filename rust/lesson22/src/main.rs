fn main() {
    println!("Hello, Generics!");

    let integer = Point {x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.5 }; 

    println!("{:?}, {:?}", integer, float);
    println!("x = {}", integer.x());

    let cs = vec!['a', 'z', 'A', 'b', 'E', 'f'];
    let is = vec![5, 10, 7, 8, 38, 33, 94, 57 ];

    println!("{}", largest_i32(&is));
    println!("{}", largest_char(&cs));
}


fn largest_i32 (list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char (list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//Generic largest
//seems very similar to java generics
//use the < > notation to define the 
//expected type to be generic 
//note: the below generic function is not 
//going to compile yet... see traits
/*
fn largest<T> (list: &[T]) -> T {
   let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}*/

//Generics can also be used in Data Types
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

//we can also impl methods with generics:
impl<T> Point<T> {
// note that we need to specify <T> right after impl keyword
//that is because not always the generic parameter in the method
//needs to be the same as the one in the struct 
    fn x (&self) -> &T {
        &self.x
    }
} 

//Example .. method with diff generic param:
struct Point2<T, U> {
    x: T,
    y: U
}

impl<T, U> Point2<T, U> {
//note that T,U are declared after impl sine they go with the struct
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
    //note that V,W are declared within the method since they are
    //only relevant here
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

/* 
    Generics can also be used with enums...
    for example the option and result are enums defined as follows:alloc

    enum Option<T> {
        Some(T),
        None
    }

    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    Due to the way generics are implemented in Rust
    there is no performance penalty for using it because 
    Rust does a "monomorphization" of the code at compile time.

    monomorphization is a process of turning generic into specific code
    the compiler looks at all the places the method is being called 
    and generate the code for the concrete types that the generic code is called with
*/