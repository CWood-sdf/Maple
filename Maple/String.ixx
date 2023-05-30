export module String;
import <string>;
import <vector>;
import <map>;
export class String {
	std::string str;
public:
	String() {
		str = "";
	}
	String(std::string str) {
		this->str = str;
	}
	String(char c) {
		str = c;
	}
	String(char* c) {
		str = c;
	}
	String(const char* c) {
		str = c;
	}
	String(int i) {
		str = std::to_string(i);
	}
	String(float f) {
		str = std::to_string(f);
	}
	String(double d) {
		str = std::to_string(d);
	}
	String(bool b) {
		str = b ? "true" : "false";
	}
	String& operator=(std::string str) {
		this->str = str;
		return *this;
	}
	String& operator=(char c) {
		str = c;
		return *this;
	}
	String& operator=(char* c) {
		str = c;
		return *this;
	}
	String& operator=(const char* c) {
		str = c;
		return *this;
	}
	String& operator=(int i) {
		str = std::to_string(i);
		return *this;
	}
	String& operator=(float f) {
		str = std::to_string(f);
		return *this;
	}
	String& operator=(double d) {
		str = std::to_string(d);
		return *this;
	}
	String& operator=(bool b) {
		str = b ? "true" : "false";
		return *this;
	}
	std::string& getReference() {
		return str;
	}
	const char& operator[](int i) const {
		return str[i];
	}
	bool operator==(std::string str) {
		return this->str == str;
	}
	bool operator==(String str) {
		return this->str == str.str;
	}
	bool operator<(const String& str) const {
		return this->str < str.str;
	}
};