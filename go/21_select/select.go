package main 

import (
	"fmt"
	"time"
)

func fibonacci (c, quit chan int) {
	x, y := 0, 1

	for {
		select { // <-- SELECT not switch :)
			case c <- x:
				x, y = y, x+y
			case <- quit :
				fmt.Println("quit")
				return
		}
	}

	// A select statement lets a go routine wait on multiple communication operations
	// A select blocks until one of the cases can run, then it executes that case.
	// it will chose one case at random if multiple are ready.
}

func main() {

	c := make(chan int)
	quit := make(chan int)

	go func() { // this borrows from Scala "implicit functions"
		for i := 0; i < 10; i++ {
			fmt.Println(<-c)
		}

		quit <- 0
	} ()

	fibonacci(c, quit)

	// Default Selection
	// default will be used if no other case is ready
	// we can use a default case to try a send / receive without blocking

	tick := time.Tick(100 * time.Millisecond)
	boom := time.After(500 * time.Millisecond)

	for {
		select {
			case <- tick:
				fmt.Println("tick.")
			case <- boom:
				fmt.Println("BOOM!")
			default fmt.Println(" . . .")
			time.Sleep(50 * time.Millisecond)
		}
	}
}
