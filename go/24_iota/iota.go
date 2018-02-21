package main 

import "fmt"


func main () {

	var t ItemType
	t = Notebook
	fmt.Println(t)
	//fmt.Println("%#v\n")


	t = Dashboard
	fmt.Println(t)
	
}

type ItemType int

//funky enum kinda of thing.... 
const (
	Notebook ItemType = iota // of type ItemType first will be zero and the rest etc
	Note
	File
	Attachment
	Dashboard
)
