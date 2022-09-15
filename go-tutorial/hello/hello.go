package main

import (
	"fmt"
	"log"

	"greetings"
)

func main() {
	log.SetPrefix("greetings: ")
	log.SetFlags(0)

	names := []string{"abc", "eee", "xyz"}
	messages, err := greetings.Hellos(names)
	if err != nil {
		log.Fatal(err)
	}

	for _, message := range messages {
		fmt.Println(message)
	}
}
