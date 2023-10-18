#pragma once

#include <string>

template <class T>
class Optional {
private:
	bool has_value;
	T data;
public:
	Optional();
	Optional(T data);
	bool hasValue();
	T getData();
	std::string toString();
};
