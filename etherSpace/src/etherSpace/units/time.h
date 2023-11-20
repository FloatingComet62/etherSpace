#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit terasecond  = unit(1, unit().setSecond(12 ));
		inline unit gigasecond  = unit(1, unit().setSecond(9  ));
		inline unit megasecond  = unit(1, unit().setSecond(6  ));
		inline unit kilosecond  = unit(1, unit().setSecond(4  ));
		inline unit hectosecond = unit(1, unit().setSecond(3  ));
		inline unit decasecond  = unit(1, unit().setSecond(2  ));
		inline unit second      = unit(1, unit().setSecond(1  ));
		inline unit decisecond  = unit(1, unit().setSecond(-1 ));
		inline unit centisecond = unit(1, unit().setSecond(-2 ));
		inline unit millisecond = unit(1, unit().setSecond(-3 ));
		inline unit microsecond = unit(1, unit().setSecond(-6 ));
		inline unit nanosecond  = unit(1, unit().setSecond(-9 ));
		inline unit picosecond  = unit(1, unit().setSecond(-12));
		inline unit femtosecond = unit(1, unit().setSecond(-15));
	};
};