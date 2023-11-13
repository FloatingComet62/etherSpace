#pragma once

#include "Core.h"

namespace etherSpace {
	class ES_API Application {
	public:
		Application();
		virtual ~Application();

		void Run();
	};

	// To be defined by external application
	Application* createApplication();
};
