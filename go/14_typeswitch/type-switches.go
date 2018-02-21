package main 

import "fmt"



// type switch is a construct that permits
// sevral type assertions in series
// it's like a regular switch statement but 
// the cases contain types (not values) 
// which are compared to the value held by the given interface value
func do(i interface{}) {
	switch v := i.(type) { // assertion on "type"
		case int:
			fmt.Printf("Twice %v is %v\n", v, v*2)
		case string:
			fmt.Printf("%q is %v bytes long\n", v, len(v))
		default:
			fmt.Printf("I don't know about type %T!\n", v)
	}
}


func main () {
	do(21)
	do("hello")
	do(true)
}
