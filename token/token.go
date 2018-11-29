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
	// For : for keyword
	For
	// If : if keyword
	If
	// Nil : nil keyword
	Nil
	// Or : or keyword
	Or
	// Print : print keyword
	Print
	// Return : return keyword
	Return
	// Super : super keyword
	Super
	// This : this keyword
	This
	// True : true literal
	True
	// Var : var keyword
	Var
	// While : while keyword
	While
	// EOF : end of file indicator
	EOF
)

// Type for token types.
// TokenType denotes the type of the token. It is to by pass the enum constrain in go.
type Type int

// Token is structure for token details.
type Token struct {
	TokenType Type
	Lexeme    string
	Line      int
}
