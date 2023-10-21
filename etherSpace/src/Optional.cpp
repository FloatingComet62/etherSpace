#include "Optional.h"

template <class T>
Optional<T>::Optional(T* data) : data(data) {
	if (data == nullptr) this->has_value = false;
	else this->has_value = true;
}

template <class T>
T* Optional<T>::getData() {
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
		// msg += this->data->toString();
		msg += ")";
	} else {
		msg += "None";
	}
}
