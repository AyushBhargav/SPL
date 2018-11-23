package lexer

import "fmt"

// Lexer scans and processes the parsed code and do lexical analysis over the code.
type Lexer struct {
}

// New instance for Lexer structure.
func New() Lexer {
	return Lexer{}
}

// Scan scans the source code to produce tokens.
func (lexer *Lexer) Scan(sourceCode string) {
	fmt.Println(sourceCode)
}

type interpreterError struct {
	line    int16
	message string
}

func (i *interpreterError) Error() string {
	return fmt.Sprintf("Line: %d, Error: %s", i.line, i.message)
}
