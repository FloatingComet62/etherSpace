#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit teramole  = unit(1, unit().setMole(12 ));
		inline unit gigamole  = unit(1, unit().setMole(9  ));
		inline unit megamole  = unit(1, unit().setMole(6  ));
		inline unit kilomole  = unit(1, unit().setMole(4  ));
		inline unit hectomole = unit(1, unit().setMole(3  ));
		inline unit decamole  = unit(1, unit().setMole(2  ));
		inline unit mole      = unit(1, unit().setMole(1  ));
		inline unit decimole  = unit(1, unit().setMole(-1 ));
		inline unit centimole = unit(1, unit().setMole(-2 ));
		inline unit millimole = unit(1, unit().setMole(-3 ));
		inline unit micromole = unit(1, unit().setMole(-6 ));
		inline unit nanomole  = unit(1, unit().setMole(-9 ));
		inline unit picomole  = unit(1, unit().setMole(-12));
		inline unit femtomole = unit(1, unit().setMole(-15));
	};
};