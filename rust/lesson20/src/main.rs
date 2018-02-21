fn main() {
    println!("Hello, Collections!");

/*
    Collection can contain multiple values
    unlike built-in array and tuples the data of 
    collections is stored in the heap.

    Each kind of collection has different capabilities and costs.
    3 oftenly used collections are:
        1. Vector
        2. String
        3. Hash Map

    PS: More types of collections here:
    https://doc.rust-lang.org/stable/std/collections/

    Vector:
    Allows to store a variable number of values next to each other
    Vec<T> is the vector type.
    Vector can only store values of the same type
    //Pretty much like Java's Vector<t>
*/
    //empty new vector:
    let vector: Vec<i32> = Vec::new();
    //               ^
    //             Type annotation
    // Because we are not inserting any values yet,
    // We need to annotate the type of our vector.
    // It's more common to create a vector that has initial values:
    let v = vec![1, 2, 3];
    // here Rust can infer the type is Vec<i32>

    let mut v2 = Vec::new();
    //here we don't need to annotate because Rust can infer from the type in 'push'
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    //if we want to change the values in the vector we need to
    //make it mutable, that is why we use mut keyword above.

    //Like any other struct a vector will be freed when it goes out of scope.
    //https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
    //When a vector get dropped all of its contents also get dropped.

    //There are 2 ways to reference values stored in a Vector
    let third1: &i32 = &v[2]; //access by index... (starts on zero)
    let third2: Option<&i32> = v.get(2);
    println!("{}  {:?}",third1, third2);

    //third1 is a direct reference
    //third2 is a Option 
    //these two different methods allows to control how the 
    //program should behave on null values
    //For example:

    //let no_exist1 = &v[100]; //causes panic
    let no_exit2 = v.get(100); // returns None
    println!("{:?}", no_exit2);

    //the first mode should be used when accessing element out of range
    //is considered fatal flaw and should crash the program
    //The second mode should be used when accessing element out of range
    //can happen occasionally under normal operations.

    //owernship and borrow rules apply to vectors as well (as defined).
    //the following code violates the ownership & borrow rules:

    //uncomment and check it out
    let mut v3 = vec![1, 2, 3, 4 , 5];
    //let first = &v3[0];
    //v3.push(6);

    //We can also store enum on vectors
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Float(10.12),SpreadsheetCell::Text(String::from("blue"))];
    println!("{:?}", row);

/*

    String:
    collection of chars
    can be of the type str (slice of string) or String
    it's actually an array of bytes
    There are other type like CString / CStr etc ... 
*/
    //Appending with PUSH
    let mut s = String::from("foo");
    s.push_str("bar"); //can push string slice
    println!("{}", s);
    s.push('!'); // can only push single chars
    println!("{}", s);

    //We can also concatenate using + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("concat!");
    let s3 = s1 + &s2; //notice reference to s2
    println!("{}",s3);//at this point s1 no longer exists

    //the method called on + operation will take ownership of s1 
    //and reference of s2, then append s2 to s1 and return.

    //For complex formating we can use format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    let s  = format!("{}-{}-{}", s1, s2, s3);
    println!("{}",s);//format does not take ownership

    //index not allowed:
    //let t = s[0]; //this will throw an error
    //the reason is that string is a Vec<u8> 
    //so non-english chars will return longer than one might think 
    let s = "Hola";
    println!("'{}' len is {}", s, s.len());
    let s ="Здравствуйте";
    println!("'{}' len is {}", s, s.len());

    /*
        The first will return 4
        the second will return 24 (we would expect 12)
        that is because the second string most char takes two bytes of space.

        Strings can be accessed in:
        bytes: the value of the letter in bytes (e.g. 104 for h)
        scalar: direct conversion from byte to unicode
        graphme clusters: pictograms / letters  that makeup the words

    */

    let slice = &s[0..4];//first four bytes of the string
    //let slice &s[0..1];//panic at runtime (because letters are 2 bytes long)
    println!("'{}' len is {}", slice, slice.len());

    //we can use chars to iterate over strings:
    for c in s.chars() {
        print!(".{}.", c);
    }
    println!("");

    //bytes method returns each raw byte
    for b in s.bytes() {
        print!(".{}.", b);
    }
    println!("");

/*
    Hash Map:
    allows to create an associated key-value pair.
    HashMap<k, V> - it stores a mapping of keys of type K 
    to values of type V.
    This mapping is done via a hash function, which determines
    how it stores the k-v into memory.
    //Pretty much like Java's HashMap

    we have to bring  HashMap into scope before we can use:
*/
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    //this will construct HashMap<String, i32>
    //HashMap also store the values on the heap

    //We can also make a HashMap from a Vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let t_scores = vec![10,50];

    let scores: HashMap<_,_> = teams.iter().zip(t_scores).collect();
    //teams and t_scores are now invalid.
    println!("{:?}", scores);

/*
    Ownership and HashMaps

    For values that implement trait Copy (like i32) the 
    values are copied into the HashMap for Owned values like 
    String the values will be moved and the hash map will 
    take ownership of those values
*/   

    //we can get values from the hashmap 
    let t = String::from("Blue");
    println!("Blue score = {:?}", scores.get(&t));
    //println!("Yellow score = {:?}", scores.get("Yellow")); //this throws an error 
    //println!("Yellow score = {:?}", scores["Yellow"]); //this throws an error 

    //This is how we can iterate over the value:
    for (k,v) in &scores {
        println!("{} - {}", k, v);
    }

    //each individual key can only have one value associated with it at a time.
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    println!("{:?}", scores1);
    scores1.insert(String::from("Blue"), 25);
    println!("{:?}", scores1);//original value overwritten

    //we can add if not exist using entry
    scores1.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores1);
    scores1.entry(String::from("Yellow")).or_insert(70); 
    println!("{:?}", scores1);
    //the or_insert method returns the value if key exists 
    //or insert if key doesn't exist

    //update a key based on previous value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);//set count to existing value or zero
        *count +=1; //increment count value (through reference)
        //the or_insert value returns a mutable reference &mut V to the value of the key
        //the * here is to deference the value in count
        //the mutable reference goes out of the scope at the end of the for loop
    }

    println!("{:?}", map);

    /*
        By default Rust uses a cryptographically secure hashing algorithm.
        However this can be changed if needed
        That is done by specifying a different Hasher, which is a type that 
        implements the BuildHasher trait
    */


}
