#include "es_pch.h"
#include "Log.h"

using namespace etherSpace;

#ifndef ES_DIST
    // https://stackoverflow.com/a/26221725/15058455
    template<typename ... Args>
    std::string string_format(const std::string& format, Args ... args) {
        int size_s = std::snprintf(nullptr, 0, format.c_str(), args ...) + 1; // Extra space for '\0'
        if (size_s <= 0) { throw std::runtime_error("Error during formatting."); }
        auto size = static_cast<size_t>(size_s);
        std::unique_ptr<char[]> buf(new char[size]);
        std::snprintf(buf.get(), size, format.c_str(), args ...);
        return std::string(buf.get(), buf.get() + size - 1); // We don't want the '\0' inside
    }

    std::string convertColorToCode(ConsoleColors color) {
	    if (color == ConsoleColors::RESET) {
		    return "\033[0m";
	    }
        return string_format("\033[1;%dm", color);
    }
#endif

void Log::info(const std::string& message) {
    #ifndef ES_DIST
        std::cout << "[";
        Log::printColoredMessage(ConsoleColors::GREEN, "Info");
        std::cout << "] " << message << "\n";
    #endif
}

void Log::warn(const std::string& message) {
    #ifndef ES_DIST
        std::cout << "[";
        Log::printColoredMessage(ConsoleColors::YELLOW, "Warn");
        std::cout << "] " << message << "\n";
    #endif
}

void Log::critical(const std::string& message) {
    #ifndef ES_DIST
        std::cout << "[";
        Log::printColoredMessage(ConsoleColors::RED, "Error");
        std::cout << "] " << message << "\n";
    #endif
    #ifdef ES_DEBUG
        __debugbreak();
    #endif

}

void Log::printColoredMessage(ConsoleColors color, const std::string& message) {
    #ifndef ES_DIST
        std::cout << convertColorToCode(color)
            << message
            << convertColorToCode(ConsoleColors::RESET);
    #endif
}
