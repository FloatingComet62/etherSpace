#include <iostream>
#include <etherSpace.h>

class Sandbox : public etherSpace::Application {
public:
	Sandbox() {
		etherSpace::Log::info("Sandbox Initialized");
	};
	~Sandbox() {

	}
};

etherSpace::Application* etherSpace::createApplication() {
	return new Sandbox();
}

	/*
	// TODO: update docs
	// TODO: write tests
	// TODO: implement a gui lib

	std::cout << "Hello etherSpace" << std::endl;

	uint8_t* pixels = {};

	auto object = new Object();
	auto transform = Components::Transform{ object, v2(10, 10) };
	auto renderer = Components::Renderer{
		object,
		&Shapes::Rectangle{ 10.f, 10.f, &transform, pixels },
		Color("#fff")
	};
	object->addComponent(&transform);
	object->addComponent(&renderer);

	auto res = object->getComponent(Components::ComponentSignature::RENDERER);
	std::cout << Components::signatureToString(res.first->signature());
	*/