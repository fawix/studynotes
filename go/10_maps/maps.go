package main 

import "fmt"

type Vertex struct {
	Lat, Long float64
}

// "map" maps keys to values 
// The zero value of a map is nil
// A nil map has no keys, nor can keys be added
// The make function returns a map of the given type
// initialized and ready for use
var m map[string]Vertex //Looks inspired by Scala but a bit diff? 

func main() {
	
	// Looks like Maps are as convoluted as Arrays/Slices ... 
	m = make(map[string]Vertex)

	m["AT&T"] = Vertex {
		40.68433, -74.39967, //what an interesting notation...
		//reminds me somewhat of Scala
	}
	fmt.Println(m["AT&T"])

	//another way to write the same is...
	var m2 = map[string]Vertex{
		"AT&T": Vertex {
			40.68433, -74.39967, 
		}.
		"Google": Vertex {
			37.42202, -122.08408, 
		}.
	}
	// map literals are like struct literals, but the keys are required
	fmt.Println(m2)

	//or ... 
	var m3 = map[string]Vertex{
		"AT&T": { 40.68433, -74.39967 },
		"Google": {37.42202, -122.08408},
	}
	fmt.Println(m3)

	//
	// Mutating Maps
	//

	m4 := make(map[string]int)

	m4["Answer"] = 42
	fmt.Println("Answer:",m4["Answer"])
	m4["Answer"] = 48
	fmt.Println("Answer:",m4["Answer"])
	delete(m, "Answer")
	fmt.Println("Answer:",m4["Answer"])
	v, ok := m4["Answer"] 
	fmt.Println("The value:",v, "Is Present?",ok)

}

