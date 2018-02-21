package main 

type User struct {
  Name string
}

func main() {
  
  u := &User{Name: "Leto"}
  println(u.Name) 
  Modify(u)
  println(u.Name) // Prints Leto

  u2 := User{Name: "Leto"}
  println(u2.Name) 
  Modify2(u2)
  println(u2.Name) // Prints Leto

  u3 := &User{Name: "Leto"}
  println(u3.Name)
  Modify3(&u3)
  println(u3.Name) // Prints Bob
}

func Modify(u *User) { // call by sharing
  u = &User{Name: "Paul"}
}

func Modify2(u User) { // call by value (copy of u)
  u.Name = "Duncan"
}

func Modify3(u **User) { //deference
 *u = &User{Name: "Bob"}
}

// None of the above are pass-by-reference as we know from C#, C/C++ & Java
