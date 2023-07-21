#include "String.h"
#include <iostream>
#include <ostream>
using std::string;
void String::assign(std::string& str) {
	if (str.empty()) {
		ref = -1;
		return;
	}
	auto it = String::refMap.find(str);
	if (it == String::refMap.end()) {
		ref = makeRef(str);
	} else {
		ref = it->second;
	}
}
void String::assign(std::string&& str) {
	if (str.empty()) {
		ref = -1;
		return;
	}
	auto it = String::refMap.find(str);
	if (it == String::refMap.end()) {
		ref = makeRef(str);
	} else {
		ref = it->second;
	}
}
void String::assign(char c) {
	assign(std::string(1, c));
}
void String::assign(char* c) {
	assign(std::string(c));
}
void String::assign(const char* c) {
	assign(std::string(c));
}
void String::assign(int i) {
	assign(std::to_string(i));
}
void String::assign(int64_t i) {
	assign(std::to_string(i));
}
void String::assign(float f) {
	assign(std::to_string(f));
}
void String::assign(double d) {
	assign(std::to_string(d));
}
void String::assign(bool b) {
	assign(b ? "true" : "false");
}
int String::makeRef(std::string& str) {
	int index = String::refVec.size();
	if (index >= (int)String::refVec.capacity()) {
		std::cout << "Reallocating vec :( " << std::endl;
	}
	String::refVec.push_back(str);
	String::refMap[str] = index;
	return index;
}
int String::makeRef(std::string&& str) {
	return makeRef(str);
}

String::String() {
	ref = -1;
}

String::String(std::string str) {
	assign(str);
}

String::String(char c) {
	assign(c);
}

String::String(char* c) {
	assign(c);
}

String::String(const char* c) {
	assign(c);
}

String::String(int i) {
	assign(std::to_string(i));
}

String::String(float f) {
	assign(std::to_string(f));
}

String::String(double d) {
	assign(std::to_string(d));
}

String::String(bool b) {
	assign(b ? "true" : "false");
}

String& String::operator=(std::string str) {
	assign(str);
	return *this;
}

String& String::operator=(char c) {
	assign(c);
	return *this;
}

String& String::operator=(char* c) {
	assign(c);
	return *this;
}

String& String::operator=(const char* c) {
	assign(c);
	return *this;
}

String& String::operator=(int i) {
	assign(std::to_string(i));
	return *this;
}

String& String::operator=(float f) {
	assign(std::to_string(f));
	return *this;
}

String& String::operator=(double d) {
	assign(std::to_string(d));
	return *this;
}

String& String::operator=(bool b) {
	assign(b ? "true" : "false");
	return *this;
}

const string String::getReference() const {
	if (ref == -1)
		return "";
	return String::refVec[ref];
}

// const char& String::operator[](int i) const {
// 	return str[i];
// }

// bool String::operator==(std::string str) const {
// 	return this->ref == String::refMap[str];
// }

bool String::operator==(String str) const {
	return this->ref == str.ref;
}

bool String::operator<(const String& str) const {
	return this->ref < str.ref;
}

namespace Strings {
	MAKE_STRING_DECL_CPP(false);
	MAKE_STRING_DECL_CPP(true);
	MAKE_SPECIAL_DECL_CPP(eq, "=");
	void init() {

		MAKE_STRING(false);
		MAKE_STRING(true);
		MAKE_SPECIAL(eq, "=");
	}
}
