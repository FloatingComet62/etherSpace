#pragma once

#include "es_pch.h"
#include "core.h"

namespace etherSpace {
	namespace units {
		inline unit teracandela  = unit(1, unit().setCandela(12 ));
		inline unit gigacandela  = unit(1, unit().setCandela(9  ));
		inline unit megacandela  = unit(1, unit().setCandela(6  ));
		inline unit kilocandela  = unit(1, unit().setCandela(4  ));
		inline unit hectocandela = unit(1, unit().setCandela(3  ));
		inline unit decacandela  = unit(1, unit().setCandela(2  ));
		inline unit candela      = unit(1, unit().setCandela(1  ));
		inline unit decicandela  = unit(1, unit().setCandela(-1 ));
		inline unit centicandela = unit(1, unit().setCandela(-2 ));
		inline unit millicandela = unit(1, unit().setCandela(-3 ));
		inline unit microcandela = unit(1, unit().setCandela(-6 ));
		inline unit nanocandela  = unit(1, unit().setCandela(-9 ));
		inline unit picocandela  = unit(1, unit().setCandela(-12));
		inline unit femtocandela = unit(1, unit().setCandela(-15));
	};
};