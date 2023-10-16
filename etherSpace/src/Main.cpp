#include <iostream>

#include "Color.h"

int main() {
	std::cout << "Hello etherSpace" << std::endl;
	Color c("#ffeeaa");
	std::cout << c.toString();
	std::cin.get();
}