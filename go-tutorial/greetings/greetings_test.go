package greetings

import (
	"regexp"
	"testing"
)

func TestHelloNmae(t *testing.T) {
	name := "Gladys"
	want := regexp.MustCompile(`\b` + name + `\b`)
	message, err := Hello("Gladys")
	if !want.MatchString(message) || err != nil {
		t.Fatalf(`Hello("Gladys") = %q, %v, want match for %#q, nil`, message, err, want)
	}
}

func TestHelloEmpty(t *testing.T) {
	message, err := Hello("")
	if message != "" || err == nil {
        t.Fatalf(`Hello("") = %q, %v, want "", error`, message, err)
	}
}
