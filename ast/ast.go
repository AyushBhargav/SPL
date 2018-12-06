package ast

import "spl/token"

// Expression denotes expression syntax tree.
type Expression struct {
	left     *Expression
	right    *Expression
	operator token.Token
}
