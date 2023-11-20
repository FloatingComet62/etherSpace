#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit petagram  = unit(1, unit().setKilogram(12 ));
		inline unit teragram  = unit(1, unit().setKilogram(9  ));
		inline unit gigagram  = unit(1, unit().setKilogram(6  ));
		inline unit megagram  = unit(1, unit().setKilogram(3  ));
		inline unit kilogram  = unit(1, unit().setKilogram(1  ));
		inline unit hectogram = unit(1, unit().setKilogram(-1 ));
		inline unit decagram  = unit(1, unit().setKilogram(-2 ));
		inline unit gram      = unit(1, unit().setKilogram(-3 ));
		inline unit milligram = unit(1, unit().setKilogram(-6 ));
		inline unit microgram = unit(1, unit().setKilogram(-9 ));
		inline unit nanogram  = unit(1, unit().setKilogram(-12));
		inline unit picogram  = unit(1, unit().setKilogram(-15));
	};
};