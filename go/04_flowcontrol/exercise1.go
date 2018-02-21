/*

Exercise: Loops and Functions
As a simple way to play with functions and loops, implement the square root function using Newton's method.

In this case, Newton's method is to approximate Sqrt(x) by picking a starting point z and then repeating:
z - (((z * z) - x) / (2 * z))

To begin with, just repeat that calculation 10 times and see how close you get to the answer for various values (1, 2, 3, ...).

Next, change the loop condition to stop once the value has stopped changing (or only changes by a very small delta). See if that's more or fewer iterations. How close are you to the math.Sqrt?

Hint: to declare and initialize a floating point value, give it floating point syntax or use a conversion:

z := float64(1)
z := 1.0

*/

package main

import (
	"fmt"
	"math"
)

func Approx (x, z float64) float64 {
	return z - (((z * z) - x) / (2 * z))
}

func Sqrt(x float64) float64 {
	var z = x
	var delta = 1.0
	var precision = 0.00000000001
	var prev = 0.0

	for delta > precision {			
		z = Approx(x,z)			
		delta = math.Abs(prev - z)	
		prev = z		
	}
	
	return z	
}

func main() {
	for i:=1; i < 100000; i+=i {
		fmt.Println("Srqt %g = %g", i, Sqrt(float64(i)))
		fmt.Println("Srqt %g = %g", i, math.Sqrt(float64(i)))
	}
}

