#include <etherSpace.h>

using namespace etherSpace;

class Sandbox : public Application {
public:
	Sandbox() {
		Log::info("Sandbox Initialized");
		units::unit x = units::meter * units::second;
		std::cout << x.toString();
	};
	~Sandbox() {}
};

Application* etherSpace::createApplication() {
	return new Sandbox();
}