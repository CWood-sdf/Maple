#include "String.h"
using std::string;

String::String() {
    str = "";
}

String::String(std::string str) {
    this->str = str;
}

String::String(char c) {
    str = c;
}

String::String(char* c) {
    str = c;
}

String::String(const char* c) {
    str = c;
}

String::String(int i) {
    str = std::to_string(i);
}

String::String(float f) {
    str = std::to_string(f);
}

String::String(double d) {
    str = std::to_string(d);
}

String::String(bool b) {
    str = b ? "true" : "false";
}

String& String::operator=(std::string str) {
    this->str = str;
    return *this;
}

String& String::operator=(char c) {
    str = c;
    return *this;
}

String& String::operator=(char* c) {
    str = c;
    return *this;
}

String& String::operator=(const char* c) {
    str = c;
    return *this;
}

String& String::operator=(int i) {
    str = std::to_string(i);
    return *this;
}

String& String::operator=(float f) {
    str = std::to_string(f);
    return *this;
}

String& String::operator=(double d) {
    str = std::to_string(d);
    return *this;
}

String& String::operator=(bool b) {
    str = b ? "true" : "false";
    return *this;
}

const string& String::getReference() const {
    return str;
}

const char& String::operator[](int i) const {
    return str[i];
}

bool String::operator==(std::string str) const {
    return this->str == str;
}

bool String::operator==(String str) const {
    return this->str == str.str;
}

bool String::operator<(const String& str) const {
    return this->str < str.str;
}
