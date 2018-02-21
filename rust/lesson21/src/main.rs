fn main() {
    println!("Hello, Error Handling!");

    /*
        Errors are groupted into two major categories
            Recoverable and Uncrecoverable

        Recoverable: where its resoanable to report the error to
        the user and retry the operation, like file not found

        Unrecoverable: usually symptoms of bugs... like trying to access
        array beyond it's end.

        Rust doesn't have exception, instead it has a value Result<T, E> for 
        recoverables errors and the panic! macro that stops execution when it encounters 
        unrecoverable errors.

        When panicking the program can unwind and cleanup
        or the program can abort.

        If the binary must be small as possible we can use abort, since unwiding and cleanup 
        is a lot more work. To abort set panic = 'abort'  in the Cargo.toml [profile] section
    */

    //red_button_do_not_press(); //panic

    let v = vec![1, 2, 3];
    //v[100]; //panic
    //run with to see backtrace:
    //RUST_BACKTRACE=1 cargo run
    
    //Using the Result type

    use std::fs::File;
    //note usually use statements are on top of the file but I am putting them here 
    //for allowing a continued information flow while reading this file

    let f = File::open("hello.txt");//throw error

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error);
        }
    }; // this example doesn't handle maching different errors 



    //What if we want to create the file in case it doesn't exist
    //but still panic in case we have permission denied or other error?
    use std::io::ErrorKind; 

    let f1 = File::open("hello1.txt");//throw error

    let f1 = match f1 {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
        /*    
            the 'if error.kind == errorkind' above is called a match guard 
            it's an extra condition on a match arm
            the ref keyword is used instead of & so the match arm does not take ownership of error
            because we still want to match it (below).
            for patterns & matches a reference and give us it's value
            ref matches the value and give us the reference
        */
            match File::create("hello1.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("There was a problem creating the file: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
    //PS: to get this to execute comment the file open above :)
    /*
        The code above is super verbose, there are shortcuts for for
        these very common constructs.
        unwrap behaves like our first match 
    */

    let f2 = File::open("hello2.txt").unwrap();

    /*
        this will return file handle if OK
        or call panic if error.
    */


    /* We can use expect if we want to choose the error method: */
    let f3 = File::open("hello3.txt").expect("Failed to open hello3.txt");

    /* both expect and unwrap are very good for prototyping 
       they offer a quick solution for PoC without worrying about 
       error handling, at the same time leaving a clear mark on the code
       to search for when the moment to make the code more robust is right 
       (i.e. moving out of prototype phase)

       They are also good for test cases, if a method fail we want the 
       whole test case to fail; panic is how a test case gets marked as a failure

       It's also appropriated to call unwrap when we have some logic that guarantees
       the result (even if the compiler can't understand) Also in cases where it's impossible 
       to throw an Err like the following line:

            "127.0.0.1".parse::<IpAddr>().unwrap();
        
        However if the input for IP came from user for example, this line would no longer be acceptable.

        Generally, it's advisible to have a code panic when it's possible that you could end up in a bad state.

    */

    /* We can also chose to propagate the error up the scope */
    throw_up();
    throw_up_too();



}

fn red_button_do_not_press() {
    panic!("You pressed the red button!");
}



use std::fs::File;
use std::io::Read;


fn throw_up () -> Result<String, std::io::Error> { //lol
    let f = File::open("OMG.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e)   => return Err(e) //this propagates the error up the scope
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_)  => Ok(s),
        Err(e) => Err(e)
    }
}

//there is a shorthand notation to acheive the return above
//it uses the operator ?
fn throw_up_too () -> Result<String, std::io::Error> { //lol2

    let mut f = File::open("OMG.txt")?; //will return if error
    let mut s = String::new();
    f.read_to_string(&mut s)?; // will return if error
    Ok(s)

    //Note that ? operator can only be used in functions that 
    //return Result enum since its defined to work in the exact
    //same way as the 

}
