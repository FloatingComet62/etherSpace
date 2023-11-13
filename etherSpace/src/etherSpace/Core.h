#pragma once

#ifdef ES_PLATFORM_WINDOWS
	#ifdef ES_BUILD_DLL
		#define	ES_API __declspec(dllexport)
	#else
		#define	ES_API __declspec(dllimport)
	#endif
#else
	#error Only Windows is currently supported
#endif
