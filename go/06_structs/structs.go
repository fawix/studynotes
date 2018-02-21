package main

import "fmt"


// struct is a collection of fields...
// Like C/C++
// a type delcaration creates the new type of the 
// struct ... C does the same but we don't need to say "type" 
type Vertex struct {
	X int
	Y int
}

var (
	v1 = Vertex{1,2}	// has type Vertex
	v2 = Vertex{X: 1}	// Y:0 is implicit
	v3 = Vertex{}		// X:0 and Y:0
	p = &Vertex{1,2}	// has type *Vertex
)


func main () {

	fmt.Println(Vertex{1,2})

	v := Vertex{1,2}
	// struct fields are accessed using a dot (as does normal structs in C++ (pointers use arrows to access the value)
	v.X = 4 // I really dislike this thing with capital letter ... is like python with spaces just stupid!
	fmt.Println(v.X,v.Y)

	fmt.Println(v1, p, v2, v3)
	
}
