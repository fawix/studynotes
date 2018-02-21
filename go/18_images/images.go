package main

import (
	"fmt"
	"image"
)

// The package image defines an image interface as such:
/*
type Image interface {
	ColorModel() color.Model
	Bounds() Rectangle 
	At(x, y int) color.Color
}
*/

// The Rectangle value of Bounds method is an image.Rectangle
// https://golang.org/pkg/image/#Image

func main () {
	m := image.NewRGBA(image.Rect(0,0,100,100))
	fmt.Println(m.Bounds())
	fmt.Println(m.At(0, 0).RGBA())
}


/*
package main

import (
	"golang.org/x/tour/pic"
	"image"
	"image/color"
)

type Image struct{
	width, height int
}

func (i Image) ColorModel() color.Model {
	return color.RGBAModel
}

func (i Image) Bounds () image.Rectangle {
	return image.Rect(0,0,i.width,i.height)	
}

func (i Image) At(x, y int) color.Color {
	v := uint8( (x^y)+50)
	return color.RGBA {v, v, 255, 255}
}


func main() {
	m := Image{ 300, 150}
	pic.ShowImage(m)
}

*/
