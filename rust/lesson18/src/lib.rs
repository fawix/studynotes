/*

    Rust has a module system that allows for organized code re-use.
    A module is a namespace that contains definitions of functions or types
    and we can choose whether those definitions are public or private

    the mod keyword declares are new module. Code within the module appears either
    immediatly following the "mod" delcaration  or in a separate file.
    By default functions, types, constants and modules are private
    pub keyword makes an item public and therefore visible outside its namespace.
    The use keyword brings modules, or the definitions inside modules into scope.

*/    


// pub keyword for public
// once the module is made public
// it can be accessed from outside
pub mod network { 
    // everything inside this block is of
    // namespace = network

    //without pub this is a protected method
    //need to put pub to access from outside
    //protected means can only be used within module
    //or by other modules in the same package (crate)
    pub fn connect() {
        println!("Hello Modules!")

    }
    // to access this function from outside the module
    // we would need to specify module and use the namespace operator ::
    // like: network::connect()

    /* Note: See mod client below */

    //we can also have modules within modules
    pub mod client { // child of network module
        pub fn connect() {

        }
    }//this can be used as network::client::connect()

}

// note that we can have multiple modules in the same lib file
pub mod client { //sibiling of network module
    pub fn connect() {

    }
}

/*
    Modules in Rust are much like Filesystems, we can split 
    a module in multiple files

    lets propose the following structure
    communicator 
        |> client1
        |> network1
            |> server1


    let's separate those in 3 different files.
*/

pub mod client1;
/*
    here we have only the declaration of the client1 module.
    rust will look for a file called client1.rs
    this is equivalent to a "include" statement
    In rust this translates to:

    mod client1 {
        //contents of client1.rs
    }

    By default Rust will only look at "lib.rs" file
    and that is why the mod client1; is defined here.
*/

//now let us extract tis to a file...
/*
mod network1 {
    fn connect() {

    }
    mod server1 {
        fn connect() {}
    }

}
*/

pub mod network1;
// check the file network1/mod.rs



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        //client::connect();// this fails
        //the reason is that we are always relative to the current module
        //this means this is looking to a client::connect within the test module
        // to solve that we can point to the root namespace with :: like:
        ::client::connect();

        //or we can go up one module with super:
        super::client::connect();
        //think of super like '..' in filesystems
    }
}
