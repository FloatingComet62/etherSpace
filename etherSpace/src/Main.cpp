#include <iostream>

#include "Object.h"
#include "Component.h"

int main() {
	std::cout << "Hello etherSpace" << std::endl;

	uint8_t* pixels = {};

	auto transform = Components::Transform{ v2(10, 10) };
	auto renderer = Components::Renderer{
		&transform,
		Shapes::Rectangle{ 10.f, 10.f, &transform, pixels },
		Color("#fff")
	};
	auto object = new Object();
	object->addComponent(&transform);
	object->addComponent(&renderer);

	auto res = object->getComponent("Renderer");
	std::cout << res.first->toString();
};