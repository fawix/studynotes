package main

// Funny import syntax lol
// the import grouped by parenthesis is called
// a "factored"  import statement
// other way to write this would be:
// import "fmt"
// import "math"
// Style practice: prefer factored imports
import (
	"fmt"
//	"math/rand"
        "math"
)
/*
 By convention the package name is the same as the last elemetn of the import path
 for instance math/rand comprises files that begin with the statement package rand.
*/



func main() {
	fmt.Println("My favorite number is", 
	rand.Intn(10))

	fmt.Printf("Now you have %g problems.", math.Sqrt(7))
	//lol I just noticed.... NO SEMICOLON ';'

	//fmt.Println(math.pi) // <-- will throw an error
	fmt.Println(math.Pi)
	//In Go a name is exported if it begins with a capital letter.
	//(i.e. Pi vs pi like in the example above) ... sounds like protected / public
	// protected instead of priviate because the lower case name can be accessed within a package
	//when importing a package we can refer only to exported names

}
//this will always generate the same number
//because it has no seed....



