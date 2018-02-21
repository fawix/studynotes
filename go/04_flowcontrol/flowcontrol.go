package main

import ( 
	"fmt"
	"math"
)

func main() {

	//Looping...

	/*

		Go has only one looping construct (!!!)
		the for loop.

		/rant
		Ok... you lost the only point you had with me Golang!
		Now Im really curious: Why the heck people like go so much???
		rant/

		The loop is pretty standard...
		init state; condition; step
		

	*/

	sum := 0 //the lazy programmer short hand... ugly
	//looks like C but without parenthesis...
	for i := 0; i < 10; i++ {
		sum += 1
	} // also curly braces are always required.

	fmt.Println(sum) //yadda yadda yadda ... 

	//also... init and step statements are optional.

	sum := 1
	for ; sum < 1000; { //but semicolons are still there =.=
		sum += sum
	}

	fmt.Println(sum) //yadda yadda yadda ... 

	sum := 1
	for sum < 1000 { //but they are not actually needed
	// looks like a while ? hm...
		sum += sum
	}//clever xD

	fmt.Println(sum) //yadda yadda yadda ... 

	//this would be an infinit loop:
	for { 
		fmt.Println("boo")
		break // can use break to exit
	}


	//If statements (see functions below)
	t := true 
	if t {
		fmt.Println(sqrt(2),sqrt(-4))
		fmt.Println(pow(3,2,10))
		fmt.Println(pow(3,3,20))
		fmt.Println(pow2(3,2,10))
		fmt.Println(pow2(3,3,20))
	}



	//Case statement
	// evalutates from top to bottom
	// stops when a case succeed
	fmt.Print("Go runs on ")
	switch os := runtime.GOOS; os {
	case "darwin":
		fmt.Println("OS X.")
	case "linux":
		fmt.Println("Linux.")
	default:
		// freebsd, openbsd,
		// plan9, windows...
		fmt.Printf("%s.", os)
	}

	//This example does not call f() if i==0
	switch i {
		case 0:
		case f():
	}

	//switch without a condition is the same as 
	//switch true
	//this can be used to replace long if-then-else chains
	t := time.Now()
	switch {
	case t.Hour() < 12:
		fmt.Println("Good morning!")
	case t.Hour() < 17:
		fmt.Println("Good afternoon.")
	default:
		fmt.Println("Good evening.")
	}

	//Defer statements
	/*
		A defer statement defers the execution of a 
		function until the surrounding function returns.
		The deferred call's argument are evaluated immediately,
		but the function call is not executed until surrounding function
		returns.		
	*/

	hellodefer()

	//Defferred calls are pushed onto a stack
	//when a function returns its deferred calls are
	//exeuted in last-in-first-out (LIFO) order
	stackeddefer()
	deferevaluation()
	//https://blog.golang.org/defer-panic-and-recover
	//the article above contains some interesting usages for defer.
	//The convention in the Go libraries is that even when a package uses panic internally, 
	//its external API still presents explicit error return values.


}

func sqrt(x float64) string {
	if x < 0 {
		return sqrt(-x) + "i"
	}

	return fmt.Sprint(math.Sqrt(x)) // haha ... sprint! STRING + PRINT
}

func pow (x, n, lim float64) float64 {
	//what is the point..... ??? ............
	if v:= math.Pow(x,n); v < lim {
	//just because variables declared within the statement are visible only 
	//within the if scope... but in such a small function is pointless
	//I can see how it can become useful in larger programs tho... hm... 
		return v
	}// they call that the "short" if statement... LOL

	return lim
}

func pow2 (x, n, lim float64) float64 {
	if v := math.Pow(x,n); v < lim {
		return v
	} else {
		fmt.Printf("%g >= %g\n", v, lim)
	}
	return lim
}

func hellodefer() {

	defer fmt.Prinln("world")
	fmt.Println("hello")
}

func stackeddefer() {
	fmt.Println("counting")

	for i := 0; i < 10; i++ {
		defer fmt.Println(i)
	}

	fmt.Println("done")
}

func deferevaluation() {
	i := 0
	defer fmt.Println(i)
	i++
	return
}
