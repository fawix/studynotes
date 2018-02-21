fn main() {
    println!("Hello, Traits!");

    /*
        Traits allow us to abstract behaviours across different types
        we can use trait bound to specify at compile time we behaviour 
        we want to use in a situation/

        Trait is similar to interfaces... with differences

        The behaviour of a type consists of the methods we call on that type
        Trait definition is a way to group methods together to define a set of 
        behaviours.


    */

    let tweet = Tweet {
        username: String::from("ipsumlorem"),
        content: String::from("foo bar baz"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet:\n {}", tweet.summary());
    println!("Author:\n {}", tweet.author_summary());

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("some content")
    };


    println!("1 new article\n {}", article.summary());
    println!("Author:\n {}", article.author_summary());

}

//we use trait keyword to declare a Trait
//followed by its name:
pub trait Summarizable { 
    //then we add the group of methods signatures
    //that compose this trait 
    fn summary(&self) -> String;


    /*
        We can also provide a default implementation to the methods in the trait
        in which case the types which implement the trait might chose to 
        keep the default or override it.

    */
    fn author_summary(&self) -> String {
        String::from("(Read more...)")
    }

    /*
        Note that it is not possible to call the default 
        implementation on the override implementation.
    */
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}


//This is how we implement a Trait
impl Summarizable for NewsArticle {
//implements Sumarizable trait for NewsArticle type
//that is how impl line should be read
    fn summary(&self) -> String {
        format!("{} by {} ({}).", self.headline, self.author, self.location)
    }

    fn author_summary(&self) -> String {
        format!("About {} ({})", self.author, self.location )
    }

}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
    Note: we may implement a trait on a type
    as long as both the type and the trait are local to our 
    crate, we are not allowed to implement external traits 
    on external types. This is called the "orphan rule".

    We have a concept named " Trait Bound", this is related to generic function
    it's the ability to define that a generic type must implement a trait to
    use the generic function
*/

pub fn notify<T: Summarizable> (item: T) {
//this says it wants a generic type T with Summarizable trait
    println!("Breaking news!!!\n{}", item.summary());
    //note how we call the method from summarizable,
    //Trait bound allows compile check to avoid
    //problems where generic type does not implement 
    //the necessary methods
}

use std::fmt::Display;
use std::fmt::Debug;
//we can define multiple trait binds using + operator
pub fn notify2<T: Summarizable + Display> (item: T) {
    println!("Breaking news!!!\n{}", item.summary());
}

//that can get cluttered with multiple types and trait bounds
//we can easily write this information using the where clause
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug 
{
    1
}


//Fixing the largest function 
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
   let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}