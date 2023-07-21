#ifndef STRING_H
#define STRING_H

#include <map>
#include <string>
#include <unordered_map>
#include <vector>
#define MAKE_STRING_DECL(str) extern String str##_str
#define MAKE_SPECIAL_DECL(str, val) extern String str##_str
#define MAKE_STRING_DECL_CPP(str) String str##_str
#define MAKE_SPECIAL_DECL_CPP(str, val) String str##_str
#define MAKE_STRING(str) Strings::str##_str = String(str)
#define MAKE_SPECIAL(str, val) Strings::str##_str = String(val)
// A class to hold tokens, in memory stores them as an int
class String {
	static inline std::unordered_map<std::string, int> refMap = {};
	static inline std::vector<std::string> refVec = {};
	int ref = -1;
	void assign(std::string& str);
	void assign(std::string&& str);
	void assign(char c);
	void assign(char* c);
	void assign(const char* c);
	void assign(int i);
	void assign(int64_t i);
	void assign(float f);
	void assign(double d);
	void assign(bool b);
	int makeRef(std::string& str);
	int makeRef(std::string&& str);

public:
	static void init() {
		refMap.clear();
		refVec.clear();
		refVec.reserve(1000);
	}
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
	const std::string getReference() const;
	// const char& operator[](int i) const;
	// bool operator==(std::string str) const;
	bool operator==(String str) const;
	// bool operator!=(std::string str) const {
	// 	return !this->operator==(str);
	// }
	bool operator!=(String str) const {
		return !this->operator==(str);
	}
	bool operator<(const String& str) const;
	int getIndex() const {
		return ref;
	}
};
namespace Strings {
	MAKE_STRING_DECL(false);
	MAKE_STRING_DECL(true);
	MAKE_SPECIAL_DECL(eq, "=");
	void init();
};
struct StringHash {
	std::size_t operator()(String const& v) const {
		return std::hash<int>()(v.getIndex());
	}
};
#endif // STRING_H
