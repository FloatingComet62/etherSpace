#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit teraampere  = unit(1, unit().setAmpere(12 ));
		inline unit gigaampere  = unit(1, unit().setAmpere(9  ));
		inline unit megaampere  = unit(1, unit().setAmpere(6  ));
		inline unit kiloampere  = unit(1, unit().setAmpere(4  ));
		inline unit hectoampere = unit(1, unit().setAmpere(3  ));
		inline unit decaampere  = unit(1, unit().setAmpere(2  ));
		inline unit ampere      = unit(1, unit().setAmpere(1  ));
		inline unit deciampere  = unit(1, unit().setAmpere(-1 ));
		inline unit centiampere = unit(1, unit().setAmpere(-2 ));
		inline unit milliampere = unit(1, unit().setAmpere(-3 ));
		inline unit microampere = unit(1, unit().setAmpere(-6 ));
		inline unit nanoampere  = unit(1, unit().setAmpere(-9 ));
		inline unit picoampere  = unit(1, unit().setAmpere(-12));
		inline unit femtoampere = unit(1, unit().setAmpere(-15));
	};
};
