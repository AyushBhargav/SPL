package token

const (
	// LeftParen : (
	LeftParen = iota
	// RightParen : )
	RightParen
	// LeftBrace : {
	LeftBrace
	// RightBrace : }
	RightBrace
	// Comma : ,
	Comma
	// Dot : .
	Dot
	// Minus : -
	Minus
	// Plus : +
	Plus
	// Semicolon : ;
	Semicolon
	// Slash : /
	Slash
	// Star : *
	Star

	// Bang : !
	Bang
	// BangEqual : !=
	BangEqual
	// Equal : =
	Equal
	// EqualEqual : ==
	EqualEqual
	// Greater : >
	Greater
	// GreaterEqual : >=
	GreaterEqual
	// Less : <
	Less
	// LessEqual : <=
	LessEqual
	// Insertion : <-
	Insertion

	// Identifier : Any variable
	Identifier
	// String : String Literal
	String
	// Number : Number Literal
	Number

	// And : &
	And
	// Class : class Keyword
	Class
	// Else : else Keyword
	Else
	// False : false keyword
	False
	// Func : func keyword
	Func
	FOR
	IF
	NIL
	OR
	PRINT
	RETURN
	SUPER
	THIS
	TRUE
	VAR
	WHILE
	EOF
)
