/*
    Enums - Enumerations
    Are best used to describe a limited space.. for example: choosing between IPv6 or IPv4
*/

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String)
}

fn main() {
    println!("Hello, Enums!");

    println!("{:?}", IpAddrKind::V4);
    println!("{:?}", IpAddrKind::V6);
    // Note that the variants of enums are namespaced under the identifier
    // and we use the :: operator to access it.

    // We could now bundle the enum in a struct to define an IP address:
    let home = IpAddr { kind: IpAddrKind::V4, address: String::from("127.0.0.1") };
    let lo   = IpAddr { kind: IpAddrKind::V6, address: String::from("::1") };
    println!("{:?}, {:?}", home, lo);

    //Or we can modify the enum to hold the value more concisely:
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let lo2   = IpAddr2::V6(String::from("::1"));
    println!("{:?}, {:?}", home2, lo2);

    /*
        Fun note:
        Even tho the standard Library does contain an IpAddr 
        definition we can still create and use our own definition without conflict.

        Because we have not yet brought the standard librar definition into our scope
    */

    //Checkout Message enum below.    
    let m = Message::Quit;
    m.call();
}

//notice how enum is allowed to contain many different types.
enum Message {
    Quit, // no associated data
    Move {x: i32, y: i32 }, // anonymous struct
    Write(String), // String
    ChangeColor(i32, i32, i32) // tuple struct
}
//This definition is similar to multiple structs definitions,
// except that all Variants are grouped under Message and there 
// is no need to use the struct keyword definition within enum
// similarly to structs we can define methods in enum using the impl block.

impl Message {
    fn call(&self) {
        println!("call method");
    }
}
