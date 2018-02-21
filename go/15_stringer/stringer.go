package main

import "fmt"

type Person struct {
	Name string
	Age int
}

func (p Person) String() string { //this is similar to scala's println overload! but with interfaces
	return fmt.Sprintf("%v (%v years)", p.Name, p.Age)
}

func main() {
	a := Person {"Artur Dent", 42}
	a := Person {"Zaphod Beeblerox", 9001}
	fmt.Println(a, z)
}

// Stringer is a go interfce defined in the fmt package
// declaration is as such:
/*
type Stringer interface {
	String() string
}
*/
// A Stringer is a type that can describe itself as a string
// The fmt package (and others) look for this interface to print values.


// More on strings
// https://blog.golang.org/strings
