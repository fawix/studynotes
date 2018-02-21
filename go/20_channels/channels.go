package main 

import (
	"fmt"
)

func main() {

	// Channels

	// Channels are a typed conduit through which
	// you can send and receive values with the channel operator '<-'
	s := []int{7, 2, 8, -9, 4, 0}

	c := make(chan int) // channels must be created before used
	// by default, sends and receives block until the other side is ready.
	// This allows goroutines to synchronize withotu explict logs or condition variables
	// the below distributes the work in two routines
	go sum (s[:len(s)/2], c)
	go sum (s[len(s)/2:], c) 

	x, y := <- c, <- c // receive from channel c
	// note the assertion operator that assigns the value to x, y
	// the data flow in the direction of the arrow
	fmt.Println(x, y, x+y)


	// Buffered Channels

	// Channels can be buffered, bu using a buffer lenght as the second argument in make
	ch2 := make(chan int, 2) // buffer of size 2
	// sends to a buffered channel block only when the buffer is full
	ch2 <- 1
	ch2 <- 2
	// receives block when the buffer is empty
	fmt.Println(<-ch2)
	fmt.Println(<-ch2)


	// Range and Close
	// A sender can close a channel to indicate that no
	// more values will be sent. 
	// like: close(c)
	// A receiver can test whether a channel has been closed by
	// assigning a second parameter to the receive expression 
	// v, ok := <- ch // ok will be false if channel is closed or if there are no more values

	c3 := make(chan int, 10)
	go fibonacci(cap(c), c)
	for i := range c { // receives the value from the channel until its closed
		fmt.Println(i)
	}

	// only the sender should close the channel
	// never the receiver (that will trigger a panic)

	// Note: channels aren't like files, you don't
	// usually need to close them.
	// Closing them is only necessary when  the receiver must be told 
	// there are no more values incoming, such as terminating a range loop.

}

func sum (s []int, c chan int) {
	sum := 0

	for _,v := range s {
		sum += v
	}

	c <- sum // send sum to channel c note the channel operator
}


func fibonacci (n int, c chan int) {
	x, y := 0, 1

	for i := 0; i < n; i++ {
		c <- x
		x, y = y, x+y
	}
	close(c)
}



/*

package main

import(
	"golang.org/x/tour/tree"
	"fmt"
)

// Walk walks the tree t sending all values
// from the tree to the channel ch.
func Walk(t *tree.Tree, ch chan int) {
//v1	
	if t.Right != nil {
		go Walk(t.Right, ch)
	}
	
	if t.Left != nil {
		go Walk(t.Left, ch)
	}
	
	ch <- t.Value
}
func Walk(t *tree.Tree, ch chan int) {
//v2
	// surprise! Functional programming! .. sort of
	// let's see if I can make this work ":)
	defer close(ch)
	var walk func (t *tree.Tree) 
	
	walk = func (t *tree.Tree) { //<-- clojure ?
		if t == nil { return }
		walk(t.Right)
		walk(t.Left)
		ch <- t.Value
	}
	
	walk(t)
}


// Same determines whether the trees
// t1 and t2 contain the same values.

func Same(t1, t2 *tree.Tree) bool {
//v1 (works with walk v1 and v2)
	ch1 := make (chan int, 10) //channel for t1
	ch2 := make (chan int, 10) //channel for t2
	
	go Walk(t1, ch1)
	go Walk(t2, ch2)
	
	m1 := make(map[int]bool, 10)
	
	for i := 0; i < 10; i++ {
		m1[<- ch1] = true
	}
	
	for i := 0; i < 10; i++ {
		if !m1[<-ch2] {
			return false
		}
	}
	
	return true
}

func Same(t1, t2 *tree.Tree) bool {
//v2 (works with walk v2)
	ch1, ch2 := make (chan int), make (chan int)
	
	go Walk(t1, ch1)
	go Walk(t2, ch2)
	
	m1 := make(map[int]bool)
	
	for i := range ch1 {
		m1[i] = true
	}
	
	for i := range ch2 {
		if !m1[i] {
			return false
		}
	}
	
	return true
}

func Same(t1, t2 *tree.Tree) bool {
//v3
	ch1, ch2 := make (chan int), make (chan int)
	
	go Walk(t1, ch1)
	go Walk(t2, ch2)
	
	m1 := make(map[int]bool)
	
	for i := range ch1 {
		m1[i] = true
	}
	
	for i := range ch2 {
		if !m1[i] {
			return false
		}
	}
	
	return true
}

func main() {
	t1, t2 := tree.New(1), tree.New(1)
	ch1, ch2 := make (chan int), make (chan int)
	
	go Walk(t1, ch1)

	for i := 0; i < 10; i++ {
		fmt.Println(<-ch1)
	}
	
	go Walk(t2, ch2)

	for i := 0; i < 10; i++ {
		fmt.Println(<-ch2)
	}
	
	fmt.Println(Same(t1, t2))
	fmt.Println(Same(t1, tree.New(2)))
}



type Tree struct {
    Left  *Tree
    Value int
    Right *Tree
}

*/

/*

package main

import (
	"fmt"
	"sync"
//	"time"
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
	// Tis' broken no longer!! Tis' not.
	// Tis still broken. It is!
	
	var crawl func (url string, depth int, fetcher Fetcher, cache *Cache, done chan bool)
	crawl = func (url string, depth int, fetcher Fetcher, cache *Cache, done chan bool) {		
		//fmt.Println("depth", depth)
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
			
			doneCrawl := make(chan bool)
			
			for _, u := range urls {
				go func (url string) {
					crawl(url, depth-1, fetcher, cache, done)
					doneCrawl <- true
				}(u)
			}
			
			for _, u := range urls {
				<- doneCrawl
				fmt.Println("read doneCrawl", u)
			}
			
			fmt.Println("sending done", url)
			done <- true
			
			
			//time.Sleep(time.Second) // gawd.... ugly
		}
	}
	
	done := make(chan bool)
	crawl(url, depth, fetcher, cache, done)
	
	for i := 0; i < depth; i++ {
		//fmt.Println("waiting i=",i)			
		<- done
	}
	//<- doneCrawl
	//fmt.Println("read done")
	//done <- true 
	//return
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
