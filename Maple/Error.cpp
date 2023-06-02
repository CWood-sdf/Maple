#include "Error.h"
void throwError(std::string message, std::size_t line) {
	std::cerr << message << std::endl;
	std::cerr << "At line: " << line << std::endl;
	exit(1);
}
