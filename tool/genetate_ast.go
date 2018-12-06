package main

import (
	"fmt"
	"os"
)

func main() {
	cmdArgs := os.Args
	if len(cmdArgs) < 2 {
		fmt.Println("Please specify the directory. Usage: generate_ast <directory>")
		return
	}
	outputDirectory := cmdArgs[2]
	typeList := []string{
		"Binary : Left *Expression, Operator token.Type, Right *Expression",
		"Grouping : expression *Expression",
		"Literal : value string, TokenType token.Type",
		"Unary : Operator token.Type, right *Expression ",
	}
	defineAST(outputDirectory, typeList, "Expression")
}

func defineAST(outputDirectory string, typeList []string, baseName string) {
	path := fmt.Scanf("%s/%s.go", outputDirectory, baseName)

}
