package main

import "fmt"

// There is a reason why these are inverted
// it is still awkward... 
// read here if interested:
// https://blog.golang.org/gos-declaration-syntax
// also note that we declare a type...
// a expresion is rather declared with :=
// x int <-- type
// x := func(a, b int) int <-- expression
// var a []int
// Expression in Go puts brackets on the left.
// x = a [1]
// quick note on Pointers
// Go still uses * for pointer notation (from C) example:
// var p *int
// x = *p
// also note that we must parenthesize the type if it starts with a *:
// (*int) (nil)
// Go declarations reads left o right.
// with that out of the way.... 
//
// functions!
func add(x int, y int) int {
	return x + y
}

// when two or more consecutive params share the type this can also be written as:
func subtract(x, y int) int {
	return x - y
}

//Multiple Results
//also, a function can return any number of results
// yay! Go gets one point with me!
func swap (x, y string) (string, string) {
	return y, x
}

//Named return values
// Go return values may be named, if so they are threated as variables defined at the 
// top of the function
// names should be used to document the meaning of the return values
func split (sum int) (x, y int) { 
	x = sum * 4 / 9
	y = sum - x
	return
// a return statemetn without argument returnes the named values, 
// this is known as "naked" return; 
// note that these can harm readability in longer functions
}

//Variables
// var statements declare a list of variables
// as in function argument lists the type is last.
// a var can be at package or function level
var c, python, java bool

//Variables with initializers
var i1, j1 int = 1, 2


func main () {
	fmt.Println(add(42, 13))
	fmt.Println(subtract(42, 13))

	a, b := swap("world!", "Oh hai,") //this is expression! xD
	fmt.Println(swap(a, b))

	fmt.Println(split(17))

	var i int
	fmt.Println(i, c, python, java)

	var c1, python1, java1 = true, false, "nope"
	fmt.Println(i1, j1, c1, python1, java1)

	//Short Variable Declarations
	//inside a function the ":=" short assighment statement
	//can be used in place of a var delcaration with implicit type
	//outside of a function every statement begins with a keyword 
	//(var, func, ...) and so the construct ':=' is not available.

	var i2, j2 int = 1, 2
	k := 3
	c2, python2, java2 := true, false, "nope"
	fmt.Println(i2, j2, k, c2, python2, java2)



}
