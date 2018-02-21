package main

import (
	"fmt"
	"math"
)

/*
	Go does not have classes (!!) - err ??
	However it's possible to define methods on types --
	/rant
	well that goes aganist the very definition of METHOD ... 
	rant/
	A "method" is a function with a special "receiver" argument.
	The receiver appears in its own argument list between the func keyword and the "method" name


*/


type Vertex struct {
	X, Y float64
}

// Abs here is the "method"
// v Vertex is the "receiver"
func (v Vertex) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

type MyFloat float64
func (f MyFloat) Abs() float64 {
	if f < 0 {
		return float64(-f)
	}

	return float64(f)
}

func main () {
	v := Vertex{3, 4}
	fmt.Println(v.Abs())
	//			 ^
	//			 looks like a method... looks can be deceiving
	// reminds me of method injection of Java

	// Doc says ....
	// Methods are Functions
	// No they aren't ... "Methods" are Functions quotes are very important
	// this go thing called method is indeed a function.
	// I will always refer to this go thingy as "methods"

	fmt.Println(Abs(v))

	// "Methods" can be declared on non-struct types too
	// see MyFloat above
	f := MyFloat(-math.Sqrt2)
	fmt.Println(f.Abs())

	// Pointers can also join the party! (see Scale below)
	fmt.Printf("Before scaling: %+v, Abs: %v\n", v, v.Abs())
	v.Scale(5)
	fmt.Printf("After scaling: %+v, Abs: %v\n", v, v.Abs())
	Scale(&v, 2)

	//pointer indirection
	// for the v.Scale statement even though v is a value
	// (not a pointer), the method with the pointer receiver is 
	// called automatically.
	// That is, Go interprets that statement as (&v),Scake(5)
	// Since the Scale method has a pointer receiver.

	// The reverse happens with the Scale() statement
	// Functions that take a value argument must take a value of that specified type.

	// while methods with value receivers take either a value or a 
	// pointer as the receiver when they are called

	// Choosing a value or pointer receiver
	// There are two reasons to sue a pointer receiver
	// 1. so that a method can modify the  value that its receiver points to
	// 2. avoid copying the value on each method call. 
	// 	  (this can be more efficient for laerge structs)
}


// This means the receiver type has the literal
// syntax *T for some type T
// (T cannot itself ve a pointer)
// meaning the "method" is defined on the pointer,
// which propagates back to the variable
// hm... I'm thinking member function is adequate
func (v *Vertex) Scale (f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}//if we delete the * the value is not propagated back

func Abs(v Vertex) float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

func Scale (v *Vertex, f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}

