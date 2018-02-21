package main 

import "fmt"


func main(){

	var pow = []int{1, 2, 4, 5, 16, 32, 64, 128, 256}
	// The range form of the for loop iterates 
	// over a slice or map.
	// when ranging over a slice, two values
	// are returned fir each interation.
	// The first is the index, the second is a copy
	// of the element that is at the index
	for i, v := range pow {
		fmt.Printf("2**%d = %d\n", i, v)
	}

	// we can skip the index or value by assigning to _
	// or the value :)

	pow := make([]int, 10)
	for i := range pow {
		pow[i] = 1 << uint(i) // ooh! << operator! Inspired by Shell ? C++ too, possibly. I Likes!
	}

	for _, value := range pow {
		fmt.Printf("%d\n", value)
	}
}
