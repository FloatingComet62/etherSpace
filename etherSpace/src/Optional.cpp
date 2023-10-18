#include "Optional.h"

template <class T>
Optional<T>::Optional() {
	this->has_value = false;
	this->data = nullptr;
}

template <class T>
Optional<T>::Optional(T data) {
	this->has_value = true;
	this->data = data;
}

template <class T>
T Optional<T>::getData() {
	return data;
}

template <class T>
bool Optional<T>::hasValue() {
	return this->has_value;
}

template <class T>
std::string Optional<T>::toString() {
	std::string msg = "";
	if (this->has_value) {
		msg += "Some(";
		msg += this->data->toString();
		msg += ")";
	} else {
		msg += "None";
	}
}
