
ackage main

import (
	"encoding/json"
	"fmt"
	"log"
)

func main() {
	fmt.Println("blah")
	fmt.Println("blah")

	m := Item{"id", "type", true, "kingdom", "title", []string{"tag1", "tag3"}, "state"}

	fmt.Printf("%#v\n", m)

	buff, err := json.Marshal(m)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s\n", string(buff))

}

type Item struct {
	ID        string   `json:id`
	Type      string   `json:type` //I want enum
	Encrypted bool     `json:encrypted`
	Kingdom   string   `json:kingdom`
	Title     string   `json:title`
	Tags      []string `json:tags`
	State     string   `json:state`
}


