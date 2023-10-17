#include <iostream>

#include "Color.h"

int main() {
	std::cout << "Hello etherSpace" << std::endl;
	Color c("#a6b3f9e4");
	std::cout << c.toString();
	std::cin.get();
}