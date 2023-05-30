export module Error;
import <iostream>;
export void throwError(std::string message, std::size_t line = 0) {
	std::cerr << message << std::endl;
	std::cerr << "At line: " << line << std::endl;
	exit(1);
}