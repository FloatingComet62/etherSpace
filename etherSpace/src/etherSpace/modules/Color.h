#pragma once

#include "../Core.h"

namespace etherSpace {
	class ES_API Color {
	public:
		uint8_t red;
		uint8_t green;
		uint8_t blue;
		uint8_t alpha;

		Color(uint8_t red = 0, uint8_t green = 0, uint8_t blue = 0, uint8_t alpha = 0);
		Color(std::string hex_string);
		std::array<uint8_t, 4> toRGBA();
		std::array<uint8_t, 4> toBGRA();
		std::string toString();
	};
};
