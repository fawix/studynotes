package main 

import (
	"fmt"
	"io" // io package specifies the Reader interface
	"os"
	"strings"
)

type rot13Reader struct {
	r io.Reader
}

// The Reader interface represetns the read end of a stream data
// Implementations in the defautl library:
// https://golang.org/search?q=Read#Global
func main() {
	r := strings.NewReader("Hello, Reader!")
	
	b := make ([]byte, 8)//will read 8 bytes at once
	for {
		n, err := r.Read(b)
		fmt.Printf("n = %v err = %v, b = %v\n", n, err, b)
		fmt.Printf("b[:n] = %q\n", b[:n])

		if err == io.EOF {
			break
		}
	}

	//A common pattern is an io.Reader that wraps another
	// io.Reader, modifying the stream in some way.
	s := strings.NewReader("Blah blah blah")
	r := rot13Reader{ s }
	io.Copy(os.Stdout, &r)

	// For example, the gzip.NewReader function takes
	// an io.Reader (a stream of compressed data) and returns a 
	// *gzip.Reader that also implements io.Reader (a stream
	// of decompressed data)


	// A = 65 Z = 90 || a = 97 z = 122




}

// The io.Reader interface has a Read method
// func (T) Read (b []byte) (m int, err error)
// Read populates th given byte slice with data
// and returns the number of bytes populated and an error value
// It returns an io.EOF error when the stream ends.

/*

package main

import "golang.org/x/tour/reader"

type MyReader struct{}

func (r MyReader) Read(b []byte) (n int, err error) {
	b[0] = 'A'
	return 1, nil
}

func main() {
	reader.Validate(MyReader{})
}

*/


/*

package main

import (
	"io"
	"os"
	"strings"
	//"fmt"
)

const abc00 string = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
const abc13 string = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm"

type rot13Reader struct {
	r io.Reader
}

func (r13 *rot13Reader) Read(b []byte) (n int, err error)  {
	
	n, err = r13.r.Read(b)	
	
	if err != nil {
		return 0, err
	}
	
	for i := 0; i < n; i++  {		
		if b[i] >= 'A' && b[i] <= 'Z' || b[i] >= 'a' && b[i] <= 'z' {
			b[i] = abc13[strings.IndexByte(abc00, b[i])]
		}
	}
	
	return n, err
}

func main() {
	s := strings.NewReader("Lbh penpxrq gur pbqr!")
	r := rot13Reader{s}
	io.Copy(os.Stdout, &r)
}


*/
