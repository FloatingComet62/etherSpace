#pragma once

#ifdef ES_PLATFORM_WINDOWS

extern etherSpace::Application* etherSpace::createApplication();

int main(int argc, char** argv) {
	auto app = etherSpace::createApplication();
	app->Run();
	delete app;
}
#endif
