//this will be a library of sorts..
// just a demo
// in go packages are really important
package stringutil

//Reverse a string
func Reverse (s string) string {
	// yep type comes AFTER the name 
	// yep return argument comes in front of function ( a bit like scala but without the : )

	r :=  [] rune(s) // dunno ?
	for i,j := 0, len(r)-1; i < len(r)/2; i,j = i+1, j-1 {
		// really weird sintax!
		r[i], r[j] = r[j], r[i]
		//this is really interesting tho.
	}

	return string(r)
}
