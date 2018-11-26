package lexer

import (
	"fmt"
	"spl/token"
)

// Lexer scans and processes the parsed code and do lexical analysis over the code.
type Lexer struct {
	sourceCode string
	tokens     []token.Token
	start      int
	current    int
	line       int
}

// New instance for Lexer structure.
func New(sourceCode string) Lexer {
	return Lexer{sourceCode: sourceCode, line: 1}
}

// Scan scans the source code to produce tokens.
func (lexer *Lexer) Scan() error {
	return nil
}

type interpreterError struct {
	line    int
	message string
}

func (i *interpreterError) Error() string {
	return fmt.Sprintf("Line: %d, Error: %s", i.line, i.message)
}

func (lexer *Lexer) scanTokens() []token.Token {
	for !lexer.isAtEnd() {
		lexer.start = lexer.current
		lexer.scanToken()
	}
	lexer.tokens = append(lexer.tokens, token.Token{
		TokenType: token.EOF,
		Lexeme:    "",
		Line:      lexer.line,
	})
	return lexer.tokens
}

func (lexer *Lexer) isAtEnd() bool {
	return lexer.current >= len(lexer.sourceCode)
}

func (lexer *Lexer) scanToken() error {
	c := lexer.advance()
	switch c {
	case '(':
		lexer.addToken(token.LeftParen)
	case ')':
		lexer.addToken(token.RightParen)
	case '{':
		lexer.addToken(token.LeftBrace)
	case '}':
		lexer.addToken(token.RightBrace)
	case ',':
		lexer.addToken(token.Comma)
	case '.':
		lexer.addToken(token.Dot)
	case '-':
		lexer.addToken(token.Minus)
	case '+':
		lexer.addToken(token.Plus)
	case ';':
		lexer.addToken(token.Semicolon)
	case '*':
		lexer.addToken(token.Star)
	default:
		// Correct it.
		return interpreterError{line: lexer.line, message: fmt.Sprintf("Token: %s, Line: %d", string(c), lexer.line)}
	}
	return nil
}

func (lexer *Lexer) advance() byte {
	lexer.current++
	return lexer.sourceCode[lexer.current-1]
}

func (lexer *Lexer) addToken(tokenType token.Type) {
	subText := lexer.sourceCode[lexer.start : lexer.current-1]
	lexer.tokens = append(lexer.tokens, token.Token{
		TokenType: tokenType,
		Lexeme:    subText,
		Line:      lexer.line,
	})
}
