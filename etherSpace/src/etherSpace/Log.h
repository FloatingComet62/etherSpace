#pragma once

#include "Core.h"

#include <string>

namespace etherSpace {
	enum ConsoleColors {
		BLACK = 30,
		RED = 31,
		GREEN = 32,
		YELLOW = 33,
		BLUE = 34,
		MAGENTA = 35,
		CYAN = 36,
		WHITE = 37,
		RESET = 0
	};

	class ES_API Log {
	public:
		static void info(const std::string& message);
		static void warn(const std::string& message);
		static void critical(const std::string& message);
	private:
		static void printColoredMessage(ConsoleColors color, const std::string& message);
	};
};
