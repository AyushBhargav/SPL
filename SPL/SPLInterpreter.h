#pragma once
#include <iostream>
#include <vector>

class SPLInterpreter
{
public:
	SPLInterpreter();
	~SPLInterpreter();
	void run(std::vector<char>);
	std::string getError();
	void clearError();
private:
	std::string error;
};
