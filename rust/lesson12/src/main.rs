
//Struct... pretty much like C/C++
//but ends with comma instead of ; (more like a list... )
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
} // and doesn't have the ending semicolon ;

fn main() {
    println!("Hello, structs!");

    //Like tuples, the fields of a struct can be of different types
    //Unlike tuples we can name each piece. Meaning structs are more flexible.
    //Once we defined a struct we need to create an instance of it to use:
    let mut user1 = User {
        email: String::from("blah@blah.com"),
        username: String::from("blah"),
        active: true,
        sign_in_count: 1
    };//obviously this is just a example using hardcoded values...

    //Access specific portions by dot-notation
    println!("{}", user1.username );

    user1.sign_in_count = 1 + user1.sign_in_count;
    println!("{}", user1.sign_in_count);

    let user2 = build_user(String::from("foo@bar.com"),String::from("foobar"));
    println!("{}", user2.username);

    let user3 = copy_user(user2);
    println!("{}", user3.sign_in_count);

    //Tuple Structs
    //without named fields, to create a different type.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    /*    Note that black and origin values are different types.    */


    //Unit-like structs
    struct Unit{}
    //.........not sure what this is for?
    //The Book says: can be useful to implement a Trait on a type 
    //               when the type dont have any data.
}


//short-hand trick to init fields
fn build_user(email: String, username: String) -> User {
    User {
        email, // instead of writing email: email
        username,
        active: true,
        sign_in_count: 4
    }
}

//Struct update syntax:
fn copy_user(user: User) -> User {
    User {
        email: String::from("blah2@blahblah.com"),
        username: String::from("foobar"),
        ..user // means... clone the rest from user
    }
}