#include "SPLInterpreter.h"


SPLInterpreter::SPLInterpreter() {
	this->error = "";
}


SPLInterpreter::~SPLInterpreter() {
}

void SPLInterpreter::run(std::vector<char> source) {
	for (std::vector<char>::iterator it = source.begin(); it != source.end(); it++) {
		std::cout << *it << std::endl;
	}
}

std::string SPLInterpreter::getError() {
	return "";
}

void SPLInterpreter::clearError() {
	this->error = "";
}