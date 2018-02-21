package main

import (
	"fmt"
	"math"
)

var (
	ToBe    bool 		= false
	MaxInt  uint64 		= 1<<64 - 1
	z  	complex128	= cmplx.Sqrt(-5 + 12i)
)

//Constants
const Pi = 3.14

func needInt(x int) int { return x*10 + 1 }
func needFloat(x float64) float64 { return x * 0.1 }

const (
	// Create a huge number by shifting a 1 bit left 100 places.
	// In other words, the binary number that is 1 followed by 100 zeroes.
	Big = 1 << 100
	// Shift it right again 99 places, so we end up with 1<<1, or 2.
	Small = Big >> 99
)

func main () {
    fmt.Printf("Type: %T Value: %v\n", ToBe, ToBe)
    fmt.Printf("Type: %T Value: %v\n", MaxInt, MaxInt)
    fmt.Printf("Type: %T Value: %v\n", z, z)


	var i int
	var f float64
	var b bool
	var s string
	fmt.Printf("%v %v %v %q\n", i, f, b, s)
	//default values are:
	//0, 0, false, "" (empty)

	/*

	Other types are:

	    - string
	    - int, int8, int16, int32, int64
	    - uint, uint8, uint16, uint32, uint64, uintptr
	    - byte (same as uint8)
	    - rune (same as int32 - represents a Unicode code point)
	    - float32, float64
	    - complex64, complex128


	    I prefer Rust's approach... but anyway

	    the types int, uint and uintptr are usually 32-bits on 32bit system and 64bits on 64-bits systems.

	*/

	//Type Conversions...
	var x, y int = 3,4
	var f float64 = math.Sqrt(float64(x*x + y*y))
	var z uint = uint(f)
	fmt.Println(x,y,z)

	/*

	The expression T(v) converts the value v to type T
	as seen above uint(f)

	Some numeric conversions:
		var i int = 42
		var f float64 = float64(i)
		var u uint = uint(f)

	Or, put more simply:
		i := 42
		f := float64(i)
		u := uint(f)

	Unlike in C, in Go assignment between items of different
	type requires an explicit conversion. Try removing the float64
	or uint conversions in the example and see what happens.


	*/

	//Type Inference
	v := 42 // change me!
	//v := "boohoo"  //can't redefine the type once its define tho
	//if we leave both lines above uncommented we get an error
	fmt.Printf("v is of type %T\n", v)

	//Constants
	const Truth = true
	fmt.Println("Truth:", Truth)
	fmt.Println("Pi:", Pi)
	/*

	constants are delcared with 'const' keyword.
	They can be a char, a string, boolean or numeric values.

	Constants cannot be delcared using the := syntax (expression)


	Numeric constants are high-precision values.
	An untyped constant takes the type needed by its context.
	*/

	fmt.Println(needInt(Small))
	//fmt.Println(needInt(Big)) //This line causes buffer overflow exception.
	//An int can store at maximum 64-bits (sometimes less)
	fmt.Println(needFloat(Small))
	fmt.Println(needFloat(Big))

}


