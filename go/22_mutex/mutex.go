package main

import (
	"fmt"
	"sync"
	"time"
)

/*

Mutex in this context stands for mutual exclusion.
Its used to ensure only one routine can access a variable at a time.
To avoid conflicts.

Go standard library provides mutual exclusion with sync.Mutex and it has
only two methods. Lock, Unlock.

We can define a block of code to be executed in mutual exclusion by
surrounding it with a call to Lock and Unlock as shown in the Inc method below.

We can also use defer to ensure the mutex will be unlocked as in the Value method.

*/

// SafeCounter is safe to use concurrently
type SafeCounter struct {
	v   map[string]int
	mux sync.Mutex
}

// Inc increments the counter for the given key
func (c *SafeCounter) Inc(key string) {
	//Lock so only one goroutine at a time can access the map c.v
	c.mux.Lock()
	c.v[key]++
	c.mux.Unlock()
}

// Value returns the current value of the counter
// for the given key
func (c *SafeCounter) Value(key string) int {
	// Lock so only the one goroutine at a time acn access the map c.v
	c.mux.Lock()
	defer c.mux.Unlock()

	return c.v[key] // why lock to read????
}

func main() {
	c := SafeCounter{v: make(map[string]int)}
	for i := 0; i < 1000; i++ {
		go c.Inc("somekey")
	}

	time.Sleep(time.Second)
	fmt.Println(c.Value("somekey"))
}
