fn main() {
    let number = 3;

    // construct fairly normal:
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // all comon stuff like the dreaded chaining of ifs is also supported
    // can also do ... if variable  { ... } 
    // overall behaves like C/C++

    //if + let 
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        //"six" // <-- note that returning different types is not allowed this throws an error.
    }; // notice the semicolon here
    // the entire if condition block is an statement
    // that return expressions depending on it being true or false
    println!("number: {}", number);
    //Loops (can be loop while or for)

    loop {
        println!("again!"); 
        break; //will loop forever without break
    }

    // while :
    let mut n1 = 3;

    while n1 != 0 {
        println!("{}!", n1);
        n1-=1;
    }

    // for offers an efficient way to navigate a collection
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //Another way to use for is interating over a range
    for n in (1..4).rev() {
        println!("{}!!", n);
    }//note that the upper bound is not inclusive
}
