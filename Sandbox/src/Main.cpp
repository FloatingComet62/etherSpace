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