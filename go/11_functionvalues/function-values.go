package main

import (
	"fmt"
	"math"
)

// declares that will receive a function that takes two float64 and returns one
func compute (fn func(float64, float64) float64) float64 {
	return fn(3,4)
}

func main() {
	hypot := func(x,y float64) float64) {
		return math.Sqrt(x*x,y*y)
	}

	fmt.Println(hypot(5,12))

	// Functions can also be passed as arguments
	// they will behave like values
	fmt.Println(compute(hypot))
	fmt.Println(compute(math.Pow))
	fmt.Println(compute(math.Mod))

	//Functions as Closures
	//Go functions may e closures. A closure is a 
	// function value that references variables from 
	// outside its body.
	// The function may access and assign to the referenced variables
	// In this sense, the function is bound to the variables (like ... global scope xP )

	pos, neg := adder(), adder() // phew two variables with functions inside .. reminds me of scala 
	for i := 0; i < 10; i++ {
		fmt.Println(
			pos(i),
			neg(-2*i)
		)
	}

	//Closures can also become somewhat of a "global"  scope
	//but "self contained"  the following is a good example:
	f := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(f())
	}


}

// it returns a function o.o (!!)
// it's clever reminds me of callbacks on javascript but a bit diff
func adder() func(int) int {
	sum := 0
	return func (x int) int {
		sum += x
		return sum
	}
}


// fibonacci is a function that returns
// a function that returns an int.
func fibonacci() func() int {
	var n1, n2 int = 1, 1	// Ah! Now I get it :)
	return func () int { 
		fib := n1 + n2
		n1, n2 = n2, fib
		return fib
	}
}

