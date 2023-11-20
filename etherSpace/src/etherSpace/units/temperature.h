#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit terakelvin  = unit(1, unit().setKelvin(12 ));
		inline unit gigakelvin  = unit(1, unit().setKelvin(9  ));
		inline unit megakelvin  = unit(1, unit().setKelvin(6  ));
		inline unit kilokelvin  = unit(1, unit().setKelvin(4  ));
		inline unit hectokelvin = unit(1, unit().setKelvin(3  ));
		inline unit decakelvin  = unit(1, unit().setKelvin(2  ));
		inline unit kelvin      = unit(1, unit().setKelvin(1  ));
		inline unit decikelvin  = unit(1, unit().setKelvin(-1 ));
		inline unit centikelvin = unit(1, unit().setKelvin(-2 ));
		inline unit millikelvin = unit(1, unit().setKelvin(-3 ));
		inline unit microkelvin = unit(1, unit().setKelvin(-6 ));
		inline unit nanokelvin  = unit(1, unit().setKelvin(-9 ));
		inline unit picokelvin  = unit(1, unit().setKelvin(-12));
		inline unit femtokelvin = unit(1, unit().setKelvin(-15));
	};
};