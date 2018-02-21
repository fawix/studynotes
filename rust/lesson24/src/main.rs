fn main() {
    println!("Hello, Lifetimes Intro!");

    //Lifetimes = how long a reference can live
    //we can extend and reference lifetimes in Rust
    //PS: I think this is very unique to Rust

    //In essence, lifetimes are a measure to prevent 
    //dangling references at compile time - enforcing memory safety.

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());

    println!("The longest is {}", result);

    /*
        Lifetimes take deeper meaning when we have a varitety of scopes
        if we had an inner scope for string2 and used the result in an outter scope
        for example, it would no longer satisfy the condition layed out
        by the function lifetime.
    */

    //See excerpt struct below
    let novel = String::from("Something. Something 2... ");
    let first_sentence = novel.split('.')
                              .next()
                              .expect("Could not find a '.' in sentence");
    let i = Excerpt{ part: first_sentence };

    println!("Excerpt: {:?}", i);

    /*
        Anytime our function or structs uses a reference / returns
        a reference we need to specify the lifetime of the reference
        to prevent dangling references.

        Some common patterns do not require the lifetimes to be annotated,
        because Rust can infer those, they are refered as "lifetime elison rules".

        Lifetimes on functions or methods parameters are called input lifetimes
        and on the return values are called output lifetimes. The compile will
        try to infer the lifetimes based on these tree rules

            1. Each input parameter that is a reference gets its own
            lifetime.

            2. if there is exactly on input lifetime then that lifetime is applied
            to all output lifetime return values

            3. if there are multiple input lifetime parameters but one of them is &self
            or &mut self then the lifetime of self is assigned to all output lifetime.
    */

    /*
        There is one special type of lifetime which is called the 
            Static Lifetime

        It's annotated with 'static
        all string literals have the 'static lifetime
    */

    let s: &'static str ="I have a static lifetime.";

    /*
        before using static lifetimes think about whether the variable
        being annotated actually lives for the entirety of the program or not
        sometimes it's a simple matter of fixing a dangling reference or a lifetime
        mismatch instead.
    */


}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
//lifetime annotation is a variable name preceeded by ' 
//commonly short name, &'a is a reference to lifetime 'a
//the signature above tells rust that both x and y must live as 
//long as retuned result.
//we also need to annotate the lifetime on the method signature
//so it knows beforehand to compute that information
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
    This function requires a generic lifetime parameter
    so Rust can know to either extend X or Y refence

    Lifetime annotation can also be applied to struct definitions
*/
#[derive(Debug)]
struct Excerpt<'b> {
    part: &'b str
}

//example where the 3rd rule of lifetime inference applies:
impl<'b> Excerpt<'b> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//Lifetime + Trait + Generic
use std::fmt::Display;
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
//both the generic declaration of T and the lifetime 'a goes into a list inside the angle brackets
    where T: Display
{
    println!("Announcement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

