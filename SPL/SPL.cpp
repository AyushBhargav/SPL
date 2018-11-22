#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include "SPLInterpreter.h"


void runFile(std::string path, SPLInterpreter &interpreter) {
	std::ifstream sourceFile(path, std::ios::ate);
	unsigned int size = sourceFile.tellg();
	sourceFile.seekg(0, std::ios::beg);

	std::vector<char> buffer(size);
	if (sourceFile.read(buffer.data(), size)) {
		interpreter.run(buffer);
		// Abort if errors are found.
		if (interpreter.getError().compare("") != 0) {
			exit(65);
		}
	}
	else {
		throw "File couldn't be read.";
	}
}

void runREPL(SPLInterpreter &interpreter) {
	std::vector<char> buffer;
	std::cout << "Welcome to SPL REPL. Created by Ayush Bhargav." << std::endl;;
	while (true) {
		std::cout << ">> ";
		char ch;
		while (std::cin.get(ch) && ch != '\n') {
			buffer.push_back(ch);
		}
		interpreter.run(buffer);
		// Error reporting
		std::cout << interpreter.getError() << std::endl;
		buffer.clear();
	}
}

int main(int argc, char* argv[]) {
	if (argc > 2) {
		std::cout << "Error: Lots of arguement. Usage: spl [file.spl]";
		return 64;
	}
	SPLInterpreter interpreter;
	if (argc == 2) {
		runFile(std::string(argv[1]), interpreter);
	}
	else {
		runREPL(interpreter);
	}
}