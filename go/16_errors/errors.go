package main

import (
	"fmt"
	"time"
)

// Go expresses error with values 
// (duh... lots of other languages too...)

type MyError struct {
	When time.Time
	What string
}



// The Error type is a built-in interface similar to fmt.Stringer
// the fmt package also looks for the error interface when printing values
func (e *MyError) Error() string {
	return fmt.Sprintf("at %v, %s", e.When, e.What)
}


func run () error {
	return &MyError {
		time.Now()
		"Oh, Ooops!"
	}
}

func main () {
	// functions often return an error value, calling code
	// should handle errros gracefully, by testing whether the 
	// error is nil (like below)
	if err := run(); err != nil {
		fmt.Println(err)
	}

	// a nil error denotes success a non nil error denotes failure. 
}

/*

package main

import (
	"fmt"
	"math"
)

type ErrNegativeSqrt float64

func (e ErrNegativeSqrt) Error() string {
	return fmt.Sprintf("cannot Sqrt negative number: %v", float64(e)) // I believe the infinit loop is caused by 
	//the fact that fmt will look for the Error() string impl 
	// if we don't convert to value first causing 
	// recurring stack for the function
	
}

func Approx (x, z float64) float64 {
	return z - (((z * z) - x) / (2 * z))
}

func Sqrt(x float64) (float64, error) {	
	if x < 0 {
		return 0, ErrNegativeSqrt(x)
	}
	
	var z = x
	var delta = 1.0
	var precision = 0.00000000001
	var prev = 0.0

	for delta > precision {			
		z = Approx(x,z)			
		delta = math.Abs(prev - z)	
		prev = z		
	}
	
	return z, nil
}


func main() {
	fmt.Println(Sqrt(2))
	fmt.Println(Sqrt(-2))
}


*/
