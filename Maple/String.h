#ifndef STRING_H
#define STRING_H

#include <string>
#include <vector>
#include <map>

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
	std::string& getReference();
	const char& operator[](int i) const;
	bool operator==(std::string str);
	bool operator==(String str);
	bool operator<(const String& str) const;
};

#endif // STRING_H
