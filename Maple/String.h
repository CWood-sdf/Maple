#ifndef STRING_H
#define STRING_H

#include <map>
#include <string>
#include <vector>

class String {
	std::string str;

public:
	String();
	String(std::string str);
	String(char c);
	String(char* c);
	String(const char* c);
	String(int i);
	String(float f);
	String(double d);
	String(bool b);
	String& operator=(std::string str);
	String& operator=(char c);
	String& operator=(char* c);
	String& operator=(const char* c);
	String& operator=(int i);
	String& operator=(float f);
	String& operator=(double d);
	String& operator=(bool b);
	const std::string& getReference() const;
	const char& operator[](int i) const;
	bool operator==(std::string str) const;
	bool operator==(String str) const;
	bool operator!=(std::string str) const {
		return !this->operator==(str);
	}
	bool operator!=(String str) const {
		return !this->operator==(str);
	}
	bool operator<(const String& str) const;
};
struct StringHash {
	std::size_t operator()(String const& v) const {
		return std::hash<std::string>()(v.getReference());
	}
};
#endif // STRING_H
