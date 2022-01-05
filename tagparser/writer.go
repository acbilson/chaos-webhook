package main

import (
	"fmt"
	"io/ioutil"
)

func writeToFile(content []byte, output string) {
	err := ioutil.WriteFile(output, content, 0644)
	if err != nil {
		fmt.Println("failed to write to file", err)
		return
	}
}
