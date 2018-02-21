fn main() {
    let x = (let y = 6)
}

/*

When compile this will throw the following exception:

Compiling lesson06 v0.1.0 (file:///workspace/Rust/lesson06)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6)
  |              ^^^
  |
  = note: variable declaration using `let` is a statement

error: aborting due to previous error


A statement do not return values (let x = 6 is a statement)
A expression returns a value.

The above is complaining that it was expecting an expression but found an statement,
and as statements have no return value the whole line is invalid. This is different than C / C++ and Ruby
where assignments returns the value of assignment.

Experssions evaluate to something and make up most of the code that is written in a Rust program.
for example, 5 + 6 is an expression (returns 11)

Note that statements can contain expressions in fact, in " let x = 6 " example given above, 
the 6 is an expression (it returns the value 6) or exemples of expressions is calling a function, macro and
the scope block {} as well.

*/

fn scopeBlockExample () {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    } // this block is the expression
    // note the last line doesn't have a semicolon ';'
    // when expressions do not include the final semicolon,
    // when adding it, the line becomes a statement instead
    // ... kinda ugly, I think this is how they decided to imply the return.
}