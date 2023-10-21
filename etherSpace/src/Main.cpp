#include <iostream>

#include "Optional.h"

template <class T>
void if_some_print(Optional<T> op) {
	if (op.hasValue()) {
		std::cout << *op.getData() << "\n";
	}
}

int main() {
	std::cout << "Hello etherSpace" << std::endl;
	int x = 5;
	auto y = Optional<int>(&x);
	auto z = Optional<int>();

	if_some_print(y);
	if_some_print(z);
}