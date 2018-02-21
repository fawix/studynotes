//this defines the network1 module
pub fn connect() {
}
//mod server1 { // submodule of network1
//    fn connect() {
//    }
//}
/*
    Because server1 is a submodule instead of a module we 
    cannot extract it in the exact same way we did with client1
    and network1. (that is, use mod server1 and create a file src/server1.rs )

    We need instead to create a new directory with the name of the parent module (network1)
    and then place our new file inside like src/network1/server1.rs

    Also this file must be moved to the network1 folder as well, and renamed to mod.rs 
    (note previously was src/network.rs)
*/

pub mod server1;


/*
    Rules for Module Filesystem

    1. if a module named foo has no submodules use src/foo.rs
    2. if a module named foo has submodules use src/foo/mod.rs
    3. these rules apply recursively.
*/