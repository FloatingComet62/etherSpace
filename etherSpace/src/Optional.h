#pragma once

#include <string>

template <class T>
class Optional {
	bool has_value;
	T* data;

public:
	Optional(T* data = nullptr);
	bool hasValue();
	T* getData();
	std::string toString();
};
