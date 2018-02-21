package main

import (
	"fmt"
	"github.com/fawix/stringutil"
)

func main() {
	fmt.Printf(stringutil.Reverse("!oG ,olleH"))
}

/*

Go command executables are statically linked; 
the package objects need not be present to run Go programs.

The first statement in a Go source file must be "package name" where name 
is the package's default name for imports. (All files in a package must use the same name.)

Go's convention is that the package name is the last element of the import path: 
the package imported as "crypto/rot13" should be named rot13.

Executable commands must always use package main.

There is no requirement that package names be unique across all packages 
linked into a single binary, only that the import paths (their full file names) be unique.

*/
