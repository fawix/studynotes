/* 

 [n]T is an array of n values of type T
 
 var a = [10]int 
 a is an array of 10 integeres.

 An array's length is part of its type and cannot be resized (???)


*/

package main 

import ( 
	"fmt"
	"strings"
)

func main () {
	var a [2]string
	a[0] = "Hello"
	a[1] = "World"

	fmt.Println(a[0], a[1])
	fmt.Println(a)

	primes := [6]int{2,3,5,7,11,13}
	fmt.Println(primes)

	//slices
	// a slice is dynamically sized
	// generally slices are more common than arrays.
	// the type []T is a slice of T (i.e []int is a slice of ints)
	var s []int = primes [1:4] // you can also create slices of arrays
	fmt.Print(s)

	// slices are like references to arrays
	// slice does not store any data, it just describes a section
	// of an underlying array (??)
	// ..... I wonder if every slice must be tied to an array?

	names := [4]string {
		"Joe",
		"Paul",
		"George",
		"Ringo"
	}

	fmt.Println(names)

	a := names [0:2]
	b := names [1:3]
	fmt.Println(a,b)

	b[0] = "XYZ"
	fmt.Println(a,b)
	fmt.Println(names)

	//Slice Literals
	//is like an array literal without the length
	q := []int{2, 3, 5, 7, 11, 13}
	fmt.Println(q)

	r := []bool{true, false, true, true, false,true}
	fmt.Println(r)

	s1 := []struct{
		i int,
		b bool
	}{
		{2, true},
		{3, false},
		{5, true},
		{7, true},
		{11, false},
		{13, true},
	}
	fmt.Println(s1)

	// when slicing you may omit lower or higher bounds to use the defaults
	// the default value of high bound is the lenght of the slice
	// the default value of the low bound is zero.

	fmt.Println(q)
	q1 := q[1:4]
	fmt.Println(q1)

	// Slicing is done by specifying a half-open range with two indices separated by a colon. 
	// For example, the expression b[1:4] creates a slice including elements 1 through 3 of b 
	// (the indices of the resulting slice will be 0 through 2).

	q1 = q1[:2]
	fmt.Println(q1)

	q1 = q1[1:]
	fmt.Println(q1)

	// a slice has both length capacity 
	// length of a slice is the number of elements it contains
	// capacity of a slice is the number of elements in the underlying array
	// counting from the first element in the slice
	// the length and capacity of a lice s can be obtained using the expressions
	// len(s) and cap(s).
	
	q2 := q
	printSlice(q2)

	q2 := q2[:0] // zero length
	printSlice(q2) // extend length
	q2 := q2[:4]
	printSlice(q2)
	q2 := q2[2:] // drop first 2 values
	printSlice(q2)

	// Nil slices 
	// the zero value of a slice is nil
	// a nil slice has length and capacity of zero
	// and has no underlying array.
	var s2 []int
	printSlice(s2)
	if s == nil {
		fmt.Println("nil!")
	}

	// Slices and Make
	// slices can be created with the built-in make function
	// this is how we create dynamicalyl-sized arrays
	// the make function allocates a zeroed array and returns a slie that refers to that array
	a := make([]int, 5) //len(a)=5
	printSlice(a)

	b := make ([]int, 0, 5)//specify capacity by passing a third argument
	printSlice(b)
	// note when the capacity is ommited as the 'a' it defaults to the len value.

	c := b[:2]
	printSlice(c)

	d := c[2:5]
	printSlice(d)


	// slices can contain any type including other slices
	board := [][]string {
		//tic-tac-toe
		[]string{"_", "_", "_"},
		[]string{"_", "_", "_"},
		[]string{"_", "_", "_"},
	}

	//The players take turns
	board[0][0] = "X"
	board[2][2] = "O"
	board[1][2] = "X"
	board[1][0] = "O"
	board[0][1] = "X"

	for i:=0; i < len(board); i++ {
		fmt.Printf("%s\n", strings.Join(board[i], " "))// oh they took this one from Scala :)
	}

	// appending to a slice
	// it is common to append new elements to a slice
	// the built-in append is used for that
	var s3 []int
	printSlice(s)

	// append works on nil slices
	// the first is a slice of type T the rest are T values.
	s3 = append (s3,0)
	printSlice(s3)

	// The slice grows as needed
	s3 = append (s3,1)
	printSlice(s3)

	// Append more than one...
	s3 = append(s3, 2,3,4,5)
	printSlice(s3)
	// If the backing array of s is too small to 
	// fit all given values a bigger array will be
	// allocated. The returned slice will point to a newly 
	// allocated array.
	// Learn more:
	// https://blog.golang.org/go-slices-usage-and-internals
	// Looks like in Go an array variable actually denotes the entire array
	// and not a pointer (unlike C). However each position does occupy its 
	// own memory space laid out sequentially.
	// you can also have the compiler count the array like so:
	// b := [...]string{"Hello", "World"}
	//instead of
	// b := [2]string{"Hello", "World"}
	// 
	// A slice is a descriptor of an array segment. It consists of a pointer to the array
	// the length of the segment and its capacity (maximum length of the segment).
	/*
		Memory stucture of "make([]byte, 3)" is:

			
			+------+
			|   +---------------+
			|      |            |
			+------+            |
			|  3   |        +---v---+-------+------+
			|      |        |       |       |      |
			+------+        |       |       |      |
			|  3   |        +-------+-------+------+
			|      |
			+------+			array (somewhere over the rainbow)
		bytes in memory	

		A slice cannot be grown beyond its capacity. Attempting to do so will cause a runtime panic, 
		just as when indexing outside the bounds of a slice or array. 
		Similarly, slices cannot be re-sliced below zero to access earlier elements in the array.

		To increase the capacity of a slice one must create a new, larger slice and copy the contents 
		of the original slice into it. This technique is how dynamic array implementations from other 
		languages work behind the scenes. 
		
		The next example doubles the capacity of s by making a new slice, t, 
		copying the contents of s into t, and then assigning the slice value t to s:

		t := make([]byte, len(s), (cap(s)+1)*2) // +1 in case cap(s) == 0
		for i := range s {
	        t[i] = s[i]
		}
		s = t

		The copy function supports copying between slices:
		copy(t,s) // this can replace the loop above
		it also handles intersections between src and dst

		re-slicing a slice doesn't make a copy of the underlying array. 
		The full array will be kept in memory until it is no longer referenced. 
		Occasionally this can cause the program to hold all 
		the data in memory when only a small piece of it is needed.
		
	*/


	// We can also append one slice to another:
	a1 := []string {"Joe", "Paul"}
	b2 := []string {"George", "Ringo", "Pete"}
	a1 = append(a1,b1)
	printSlice(a1)

	


}

func printSlice(s []int) {
	fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s)
}
