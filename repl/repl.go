package repl

import (
	"bufio"
	"fmt"
	"os"
	"spl/lexer"
)

// Repl structure denotes REPL shell and it's respective attributes.
type Repl struct {
	line       int
	hasErrrors bool
}

// Run executes the infinite loop for REPL shell.
func (repl *Repl) Run() {
	fmt.Println("Welcome to SPL REPL. Created by Ayush Bhargav.")
	scanner := bufio.NewScanner(os.Stdin)
	for true {
		fmt.Print(">> ")
		scanner.Scan()
		if scanner.Err() != nil {
			os.Exit(1)
		}
		code := scanner.Text()
		lexer := lexer.New(code)
		if codeError := lexer.Scan(); codeError != nil {
			fmt.Println(codeError)
		}
	}
}
