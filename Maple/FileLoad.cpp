#include "FileLoad.h"
using std::operator "" s;
std::string loadFile(std::string fileName) {
	std::string ret;
	std::ifstream file(fileName);
	if (file.is_open()) {
		std::string line;
		while (std::getline(file, line)) {
			ret += line + "\n";
		}
		file.close();
	}
	else {
		throwError("Could not open file: "s + fileName);
	}
	return ret;
}
