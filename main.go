package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"spl/lexer"
	"spl/repl"
)

func main() {
	cmdArgs := os.Args
	switch len(cmdArgs) {
	case 1:
		var repl repl.Repl
		repl.Run()
	case 2:
		bytes, err := ioutil.ReadFile(cmdArgs[1])
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		sourceCode := string(bytes)
		lexer := lexer.New()
		lexer.Scan(sourceCode)
	default:
		fmt.Println("Usage: spl [file.spl]")
		os.Exit(0)
	}
}
