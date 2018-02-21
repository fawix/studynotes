package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"sync"
)

type Story struct {
	Title string
}

func main() {
	rsp, err := http.Get("https://hacker-news.firebaseio.com/v0/topstories.json")
	if err != nil {
		panic(err)
	}
	defer rsp.Body.Close()

	data, err := ioutil.ReadAll(rsp.Body)
	if err != nil {
		panic(err)
	}

	var ids []int
	if err := json.Unmarshal(data, &ids); err != nil {
		panic(err)
	}

	var cursor int
	var mutex sync.Mutex

	next := func() int {
		mutex.Lock()
		defer mutex.Unlock()
		temp := cursor
		cursor++
		return temp
	}

	wg := sync.WaitGroup{}
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func() {
			for cursor := next(); cursor < len(ids); cursor = next() {
				url := fmt.Sprintf(
					"https://hacker-news.firebaseio.com/v0/item/%d.json",
					ids[cursor],
				)
				rsp, err := http.Get(url)
				if err != nil {
					panic(err)
				}
				defer rsp.Body.Close()

				data, err := ioutil.ReadAll(rsp.Body)
				if err != nil {
					panic(err)
				}
				var story Story
				if err := json.Unmarshal(data, &story); err != nil {
					panic(err)
				}
				fmt.Println(story.Title)
			}
			wg.Done()
		}()
	}

	wg.Wait()
}
