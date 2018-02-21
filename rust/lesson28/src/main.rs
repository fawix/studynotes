use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, Closures!");

    /*
        Rust's closures are anonymous functions that 
        you can save in a variable or pass as arguments to other
        functions.

        We can create a closure in one place and then call the closure 
        to evaluate it in a different context. 

        Unlike functions, closures are allowed to capture the values from
        the scop in which they are called.
    */

    println!("{}", simulated_expensive_calculation(42)); //slow

    generate_workout(42, 10);

    generate_workout_closure(42, 10);
    generate_workout_closure(24, 5);

}

//too many calls to expensive function:
//eek, very good example of how NOT to implement something
//Try to find out why before proceeding ;) 
fn generate_workout(intensity: i32, random_number: i32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}

//many ways to fix the above code...
//for this example we'll use closures
fn generate_workout_closure (intensity: i32, random_number: i32) {
    //we could move the function call outside (eek) like this:
    //let result = simulated_expensive_calculation(intensity);
    //however that means calling it even when we don't need
    //we can instead define a closure
    let expensive_regular_closure = |num| { // <-- parameters to the closure
                                    //could pass more than one param as such: |num1, num2|
                                    //curly braces indicate the begining of the body of the closure
                                    //not required for one-liners
        println!("slowly computing something...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    //a closure won't be computed immediately.
    //it will await until its invoked... 
    //Think JS callbacks or Scala, Ruby, Go closures (although a bit different)

    //This is how we call a closure:
    expensive_regular_closure(intensity); 

    /*
        Note that closures do not require us to annotate the return type (unlike fn)
        This can be acheived because closures cannot be not part of our public api's 
        Additionally, closures are usually relevant only to a narrow context / scope,
        which facilitates type inference. However, it is possible to annotate type in 
        closures like so:  
             let expensive_closure = |num: i32| -> i32 {
                 ...
             }
        
        Closures also implement the Fn traits that enable us to give a type 
        to the closure. Giving a type to the closure allow us to encapsulate it
        in a struct in order to build a lazy evalution type. 

        Note the closure we defined above will still call the calculation function every time.
        For a lazy evaluation, check out the Cache struct definition below.

    */

    let mut expensive_lazy_closure = Cache::new(|num| {
        println!("lazily computing something...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //expensive_regular_closure(intensity) // still calls calculation every time
            expensive_lazy_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            //expensive_regular_closure(intensity)
            expensive_lazy_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                //expensive_regular_closure(intensity)
                expensive_lazy_closure.value(intensity)
            )
        }
    }

}

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("slowly computing something...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


//Encapsulating a closure of type 
//Fn(i32) -> i32 //<-- represents closure signature / type
//Caching the result in value Option<i32>
struct Cache<T> 
    where T: Fn(i32) -> i32 
//reads: where T is a closure that receives i32 and returns i32
{
    calculation: T,
    value: Option<i32>
    //note: this is a naive implementation
    //with many flaws,for example:
    //if we pass a diff value for calculation then we passed first time
    //this will return the wrong value
}


impl<T> Cache<T>
    where T: Fn(i32) -> i32 
{
    //now we implement the lazy aspect 
    //of our closure evaluation 

    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v, //in case we already have some value...
            None => {     //in case we don't
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
use std::collections::HashMap;
use std::cmp::{Eq,PartialEq};
use std::hash::Hash;
use std::fmt::Debug;
struct Cache2<T, W>
    where T: Fn(W) -> W,
          W: Eq + Hash + Copy + PartialEq + Debug
{
    function: T,
    value: HashMap<W, W>
}

impl<T, W> Cache2<T, W>
    where T: Fn(W) -> W,
          W: Eq + Hash + Copy + PartialEq + Debug
{
    fn new (function: T) -> Cache2<T, W> {
        Cache2 { 
            function, 
            value: HashMap::new()
        }
    }
/*
    fn value<'a> (&mut self, arg: &'a W) -> &'a W {
        //methods don't really need to annotate...
        #[derive(Debug)]
        let mut v = self.value.get(&arg);
        println!("v = {:?}", v);

        &arg
        //well... why do it like this when we can use
        //pattern matching
        if let Some(W) = self.value.get(&arg) {
            W
        } else {
            let v = (self.function)(arg);
            self.value.insert(arg, v);
            self.value.get(&arg).unwrap()
        }
    }
*/

    fn get(&mut self, arg: W) -> W {

        match self.value.get(&arg) {  
            Some(&v) => v,
            None     => {
                let v = (self.function)(arg);
                self.value.insert(arg, v.clone());
                v
            }
        }

        /*
        self.value.get(&arg).unwrap_or_else(|o| {
            let v = (self.function)(arg.clone());
            self.value.insert(arg.clone(), v);
            self.value.get(&arg).unwrap()
        }) // this didn't work because W is not a closure?
        //so I can't invoke it with or_else??? not sure honestly

        let r = self.value.get(&arg);
        println!("r = {:?}", r);

        match r {
            Some(&v) => &v, //well this doesn't make any sense...
            //I tried to return the reference still same problem as below
            None     => {
                self.value.insert(arg.clone(), arg);
                self.value.get(&arg).unwrap()
            }
        }

       match self.value.get(arg) {
            Some(v) => v,//this didn't work because I'm keeping a 
            //immutable reference at the same time as I have a mutable reference
            None    => {
                let v = (self.function)(arg.clone());
                self.value.insert(arg.clone(), v);
                self.value.get(&arg).unwrap()
            }
        }
    */}
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn cached2_validate_new () {
        let v: HashMap<i32, i32> = HashMap::new();
        let c = Cache2::new(|x: i32| -> i32 { x });
        assert_eq!(c.value, v);
    }

    #[test]
    fn cached2_validate_get_value () {
        let mut c = Cache2::new(|x: i32| -> i32 { x + 1 });
        let arg = 3;
        let v = c.get(arg);
        assert_eq!(v, 4);
        assert_eq!(v, 4);
    }
/*
    Closures can capture their environment
*/

    #[test]
    fn enclosed() {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;

        assert!(equal_to_x(y));

        /*
            Note that even if X is not a param to the enclosure
            it still works.  That is because the enclosure is allowed
            to use the variable X defined in its scope 
            (not applicable to funcs, see below)

            When a closure capture a value from its environment, the closure 
            uses memoy to store the values for use in the closure body (overhead).
        */
    }

    /*
    //Note how this is NOT applicable to functions:
    #[test]
    fn not_enclosed() {
        let x = 4;
        fn equal_to_x (z: i32) -> bool { z == x }
        let y = 4;

        assert!(equal_to_x(y));
    }
    //does not even compile
    */



    /*
        Closures can capture the values from their environments in 3 ways:alloc
            1. take ownership
            2. borrow immutably
            3. borrow mutably

        These methodologies are defined in the Fn traits.
            1. FnOnce => take ownership (it consumes the variable it captures from its enclosing scope)
            2. Fn => borrow immutably
            3. FnMut => borrows mutably

        When we create a closure Rust infers how we want to reference the environment, however
        we can force to take ownership by using the keyword move
    */
    #[test]
    fn mine_mine() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        //println!("Can't use X here: {:?}", x); //<- fail to compile
        //because X has been moved.
        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}