/*

 Go has poitners, and it works pretty much like C
 except that points arithmetric is not allowed.

 The type *T is a pointer to type T (i.e. *int -> pointer of int)
 The zero value of pointers is nil  //Oh why...

 the & operator generates a pointer to its operand:

 i := 42 
 p = &i // p is a pointer of i

 the * operator denotes the pointers underlying value.

 fmt.Println(*p) // read i through the pointer p
 *p = 21  // set i through the pointer p

 This is known as deferencing or indirecting.

*/

package main 

import "fmt"

func main () {

	i,j := 42, 2701

	p := &i				// point to i
	fmt.Println(*p)		// read i through the pointer
	*p = 21				// set i through the pointer
	fmt.Println(i)		// see the new value of i

	p = &j				// point to j
	*p = *p / 37		// divide the j through the pointer
	fmt.Println(j)		// see the new value of j
}
