package main 

import (
	"fmt"
	"time"
)

func say (s string) {
	for i := 0; i < 5; i++ {
		time.Sleep(100 * time.Millisecond)
		fmt.Println(s)
	}
}

func main() {
	go say("world")
	say("hello")


// A goroutine is a lightweight thread managed by the Go runtime
// go f(x, y, z)
// starts a new routine running f(x, y, z)

// The evaluation of f, x, y, and z happens in the current goroutine and the execution of f happens 
// in the new goroutine.
// evaluation = current routine
// execution  = new routine

// Go routines run in the same address space, so access to shared memory
// must be synchronized. The sync package provides useful primities for that.

}


/*
https://golangbot.com/goroutines/

How to start a Goroutine?
Prefix the function or method call with the keyword go and you will have a new Goroutine running concurrently.

Lets create a Goroutine :)

package main

import (  
    "fmt"
)

func hello() {  
    fmt.Println("Hello world goroutine")
}
func main() {  
    go hello()
    fmt.Println("main function")
}
Run program in playground

In line no. 11, go hello() starts a new Goroutine. Now the hello() function will run concurrently along with the main() function. The main function runs in its own Goroutine and its called the main Goroutine.

Run this program and you will have a surprise!

This program only outputs the text main function. What happened to the Goroutine we started? We need to understand two main properties of go routines to understand why this happens.

When a new Goroutine is started, the goroutine call returns immediately. Unlike functions, the control does not wait for the Goroutine to finish executing. The control returns immediately to the next line of code after the Goroutine call and any return values from the Goroutine are ignored.
The main Goroutine should be running for any other Goroutines to run. If the main Goroutine terminates then the program will be terminated and no other Goroutine will run.
I guess now you will be able to understand why our Goroutine did not run. After the call to go hello() in line no. 11, the control returned immediately to the next line of code without waiting for the hello goroutine to finish and printed main function. Then the main Goroutine terminated since there is no other code to execute and hence the hello Goroutine did not get a chance to run.

Lets fix this now.

package main

import (  
    "fmt"
    "time"
)

func hello() {  
    fmt.Println("Hello world goroutine")
}
func main() {  
    go hello()
    time.Sleep(1 * time.Second)
    fmt.Println("main function")
}
Run program in playground

In line no.13 of the program above, we have called the Sleep method of the time package which sleeps the go routine in which it is being executed. In this case the main goroutine is put to sleep for 1 second. Now the call to go hello() has enough time to execute before the main Goroutine terminates. This program first prints Hello world goroutine, waits for 1 second and then prints main function.

This way of using sleep in the main Goroutine to wait for other Goroutines to finish their execution is a hack we are using to understand how Goroutines work. Channels can be used to block the main Goroutine until all other Goroutines finish their execution. We will discuss channels in the next tutorial.


Advantages of Goroutines over threads
Goroutines are extremely cheap when compared to threads. They are only a few kb in stack size and the stack can grow and shrink according to needs of the application whereas in the case of threads the stack size has to be specified and is fixed.
The Goroutines are multiplexed to fewer number of OS threads. There might be only one thread in a program with thousands of Goroutines. If any Goroutine in that thread blocks say waiting for user input, then another OS thread is created and the remaining Goroutines are moved to the new OS thread. All these are taken care by the runtime and we as programmers are abstracted from these intricate details and are given a clean API to work with concurrency.
Goroutines communicate using channels. Channels by design prevent race conditions from happening when accessing shared memory using Goroutines. Channels can be thought of as a pipe using which Goroutines communicate. We will discuss channels in detail in the next tutorial.


*/



/*

package main

import (
	"fmt"
	"sync"
	"time"
)

type Fetcher interface {
	// Fetch returns the body of URL and
	// a slice of URLs found on that page.
	Fetch(url string) (body string, urls []string, err error)
}

type Cache struct {
	v map[string] bool
	mux sync.Mutex
}

func (c *Cache) Add (key string, status bool){
	c.mux.Lock()
	c.v[key] = status
	c.mux.Unlock()
}

func (c *Cache) Get (key string) bool {
	c.mux.Lock()
	defer c.mux.Unlock()
	return c.v[key]
}

// Crawl uses fetcher to recursively crawl
// pages starting with url, to a maximum of depth.
func Crawl(url string, depth int, fetcher Fetcher, cache *Cache) {
	// This implementation doesn't do either:
	if depth <= 0 {
		return
	}
	
	if !cache.Get(url) {
		body, urls, err := fetcher.Fetch(url)
		if err != nil {
			fmt.Println(err)
			return
		}
		
		fmt.Printf("found: %s %q\n", url, body)
		cache.Add(url, true)
		
		for _, u := range urls {
			go Crawl(u, depth-1, fetcher, cache)
		}
		
		time.Sleep(time.Second)
	}
	return
}

func main() {
	var cache = Cache { v: make(map[string] bool) }
	Crawl("http://golang.org/", 4, fetcher, &cache)
}

// fakeFetcher is Fetcher that returns canned results.
type fakeFetcher map[string]*fakeResult

type fakeResult struct {
	body string
	urls []string
}

func (f fakeFetcher) Fetch(url string) (string, []string, error) {
	if res, ok := f[url]; ok {
		return res.body, res.urls, nil
	}
	return "", nil, fmt.Errorf("not found: %s", url)
}

// fetcher is a populated fakeFetcher.
var fetcher = fakeFetcher{
	"http://golang.org/": &fakeResult{
		"The Go Programming Language",
		[]string{
			"http://golang.org/pkg/",
			"http://golang.org/cmd/",
		},
	},
	"http://golang.org/pkg/": &fakeResult{
		"Packages",
		[]string{
			"http://golang.org/",
			"http://golang.org/cmd/",
			"http://golang.org/pkg/fmt/",
			"http://golang.org/pkg/os/",
		},
	},
	"http://golang.org/pkg/fmt/": &fakeResult{
		"Package fmt",
		[]string{
			"http://golang.org/",
			"http://golang.org/pkg/",
		},
	},
	"http://golang.org/pkg/os/": &fakeResult{
		"Package os",
		[]string{
			"http://golang.org/",
			"http://golang.org/pkg/",
		},
	},
}



*/
