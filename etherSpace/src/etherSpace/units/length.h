#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit terameter  = unit(1, unit().setMeter(12 ));
		inline unit gigameter  = unit(1, unit().setMeter(9  ));
		inline unit megameter  = unit(1, unit().setMeter(6  ));
		inline unit kilometer  = unit(1, unit().setMeter(4  ));
		inline unit hectometer = unit(1, unit().setMeter(3  ));
		inline unit decameter  = unit(1, unit().setMeter(2  ));
		inline unit meter      = unit(1, unit().setMeter(1  ));
		inline unit decimeter  = unit(1, unit().setMeter(-1 ));
		inline unit centimeter = unit(1, unit().setMeter(-2 ));
		inline unit millimeter = unit(1, unit().setMeter(-3 ));
		inline unit micrometer = unit(1, unit().setMeter(-6 ));
		inline unit nanometer  = unit(1, unit().setMeter(-9 ));
		inline unit picometer  = unit(1, unit().setMeter(-12));
		inline unit femtometer = unit(1, unit().setMeter(-15));
	};
};