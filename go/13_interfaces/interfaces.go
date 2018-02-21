package main 

import (
	"fmt"
	"math"
)

// An interface type is defined as a set of method signatures.
type Abser interface {
	Abs() float64
}

type I interface {
	M()
}

type T struct {
	S string
}

func main () {
	
	var a Abser
	f := MyFloat (-math.Sqrt2)
	v := Vertex{3, 4}

	// A value of interface type can hold any value that implements those methods.
	a = f // a MyFloat implements Abser
	a = &v // a *Vertex implements Abser

	// In the following line, v is a Vertex (not a *Vertex)
	// and does NOT implement Abser
	// a = v //<-- fails compile

	fmt.Println(a.Abs())

	// Interfaces are implemeted implicitly. 
	// There is no explict declaration of intent like "implements" keyword
	// Implicit interfaces decouple te definition of an interface from its implementation.

	/* IMHO ... not true, if you change the definition the implementation will stop working 
	// or will just not perform what it should. 
	// This provides the separate of implementation and definition to the same extent that 
	// the "implements" keyword does.
	*/

	//see above for declarations and below for implementation
	var i I = T{"hai"}
	i.M()

	// Interfaces values can be thought of as a tuple of value and type like: (value, type)
	// An interface holds a value of a specific underlying concrete type.
	// Calling a method on an interface value executes the method of the same name on its underlying type.
	describe(i)

	i = F(math.Pi)
	i.M()
	describe(i)

	// Interfaces values with nil underlying values
	// If the concrete value inside the interface itself 
	// is nil, the method will be called with a nil receiver

	// In some languages this would trigger a null pointer exception (yep.. he is talking about Java here)
	// but in Go this is common to write methods that gracefully handle beign called with a nil receiver 
	// (as the method M below)
	// Note: an interface value that holds a nil concrete value is itself non-nil.
	var t *T
	i = t 
	describe(i)
	i.M() 

	// a nil interface value holds neither value nor concrete type
	// Calling a method on a nil interface is a run-time error because there
	// is no type inside the interface tuple to indicate which concrete method to call.
	//try commenting the statement that checks for nil below to see the exception it causes.

	// The empty interface
	// The interface type that specifies zero method is known as teh empty interface
	// An empty interface may hold values of any type
	// (every type implements at least zero methods)
	// an empty interface is used by code that handles values of unknown type
	// for example fmt.Print takes any number of arguments of type interface{}

	var i2 interface{}
	describe(i2)

	i2 = 42 
	describe(i2)

	i2 = "hello"
	describe(i2)

	// Type assertions
	// provides access to an interface value's underlying concrete values
	// t := i.(T)
	// The statement above asserts that the interfave value i holds the concrete type T
	// and assigns the underlying T value to the variable t
	// If i does not hold a T, the statement will trigger a panic.
	// To test whether an interfave value holds a specific type an assertion can return two values
	// t, ok := i.(T)
	// if i holds T then ok is true, false if otherwise.

	var i3 interface{} = "hello"

	s := i3.(string)
	fmt.Println(s)

	s, ok := (string)
	fmt.Println(s, ok)

	f, ok := i.(float64)
	fmt.Println(f, ok)

	//f = i.(float64) // panic ... note how this is ussing assignment instead of assertion
	//fmt.Println(f)

}


// This method means type T implements the interface I
// but we don't need to explicitly declare that it does so.

func (t T) M() {
	if t == nil {
		fmt.Println("<nil>")
		return
	}
	fmt.Println(t.S)
}

// So essentially when the compiler comes across an implementation
// it will infer that type T implements interface I.
// I wonder how complex interface definitions will be have like this ...


type F float64

func (f F) M() {
	fmt.Println(f)
}


func describe (i I) {
	fmt.Printf("(%v, %T)\n", i, i)
}

