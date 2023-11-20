workspace "EtherSpace"
  architecture "x86_64"

  configurations {
    "Debug",   -- Optimizations [Off]. Symbols [On] . Logs [On] .
    "Release", -- Optimizations [On] . Symbols [On] . Logs [On] .
    "Dist"     -- Optimizations [On] . Symbols [Off]. Logs [Off].
  }

outputDir = "%{cfg.buildcfg}-%{cfg.system}-%{cfg.architecture}"

project "Sandbox"
  location "Sandbox"
  kind "ConsoleApp"
  language "C++"

  targetdir ("bin/" .. outputDir .. "/%{prj.name}")
  objdir ("bin/int/" .. outputDir .. "/%{prj.name}")

  files {
    "%{prj.name}/src/**.h",
    "%{prj.name}/src/**.cpp"
  }

  includedirs {
    "etherSpace/src"
  }

  links {
    "EtherSpace"
  }

  filter "system:windows"
    cppdialect "C++17"
    staticruntime "On"
    systemversion "latest"

    defines {
      "ES_PLATFORM_WINDOWS"
    }
  
  filter "configurations:Debug"
    defines "ES_DEBUG"
    symbols "On"
  
  filter "configurations:Release"
    defines "ES_RELEASE"
    optimize "On"

  filter "configurations:Dist"
    defines "ES_DIST"
    optimize "On"

project "Glfw"
	location "Vendor/glfw/"
	kind "StaticLib"
	language "C"
	staticruntime "off"
	warnings "off"

	targetdir ("bin/" .. outputDir .. "/%{prj.name}")
	objdir ("bin/int/" .. outputDir .. "/%{prj.name}")

	files {
		"Vendor/glfw/include/GLFW/glfw3.h",
		"Vendor/glfw/include/GLFW/glfw3native.h",
		"Vendor/glfw/src/glfw_config.h",
		"Vendor/glfw/src/context.c",
		"Vendor/glfw/src/init.c",
		"Vendor/glfw/src/input.c",
		"Vendor/glfw/src/monitor.c",

		"Vendor/glfw/src/null_init.c",
		"Vendor/glfw/src/null_joystick.c",
		"Vendor/glfw/src/null_monitor.c",
		"Vendor/glfw/src/null_window.c",
		"Vendor/glfw/src/platform.c",
		"Vendor/glfw/src/vulkan.c",
		"Vendor/glfw/src/window.c",
	}

	filter "system:linux"
		pic "On"

		systemversion "latest"
		
		files {
			"Vendor/glfw/src/x11_init.c",
			"Vendor/glfw/src/x11_monitor.c",
			"Vendor/glfw/src/x11_window.c",
			"Vendor/glfw/src/xkb_unicode.c",
			"Vendor/glfw/src/posix_module.c",
			"Vendor/glfw/src/posix_time.c",
			"Vendor/glfw/src/posix_thread.c",
			"Vendor/glfw/src/posix_module.c",
			"Vendor/glfw/src/glx_context.c",
			"Vendor/glfw/src/egl_context.c",
			"Vendor/glfw/src/osmesa_context.c",
			"Vendor/glfw/src/linux_joystick.c"
		}

		defines {
			"_GLFW_X11"
		}

	filter "system:macosx"
		pic "On"

		files {
			"Vendor/glfw/src/cocoa_init.m",
			"Vendor/glfw/src/cocoa_monitor.m",
			"Vendor/glfw/src/cocoa_window.m",
			"Vendor/glfw/src/cocoa_joystick.m",
			"Vendor/glfw/src/cocoa_time.c",
			"Vendor/glfw/src/nsgl_context.m",
			"Vendor/glfw/src/posix_thread.c",
			"Vendor/glfw/src/posix_module.c",
			"Vendor/glfw/src/osmesa_context.c",
			"Vendor/glfw/src/egl_context.c"
		}

		defines {
			"_GLFW_COCOA"
		}

  filter "system:windows"
		systemversion "latest"

		files {
			"Vendor/glfw/src/win32_init.c",
			"Vendor/glfw/src/win32_joystick.c",
			"Vendor/glfw/src/win32_module.c",
			"Vendor/glfw/src/win32_monitor.c",
			"Vendor/glfw/src/win32_time.c",
			"Vendor/glfw/src/win32_thread.c",
			"Vendor/glfw/src/win32_window.c",
			"Vendor/glfw/src/wgl_context.c",
			"Vendor/glfw/src/egl_context.c",
			"Vendor/glfw/src/osmesa_context.c"
		}

		defines  { 
			"_GLFW_WIN32",
			"_CRT_SECURE_NO_WARNINGS"
		}

	filter "configurations:Debug"
		runtime "Debug"
		symbols "on"

	filter { "system:windows", "configurations:Debug-AS" }	
		runtime "Debug"
		symbols "on"
		sanitize { "Address" }
		flags { "NoRuntimeChecks", "NoIncrementalLink" }

	filter "configurations:Release"
		runtime "Release"
		optimize "speed"

    filter "configurations:Dist"
		runtime "Release"
		optimize "speed"
        symbols "off"

project "EtherSpace"
  location "etherSpace"
  kind "SharedLib"
  language "C++"

  targetdir ("bin/" .. outputDir .. "/%{prj.name}")
  objdir ("bin/int/" .. outputDir .. "/%{prj.name}")

  pchheader "es_pch.h"
  pchsource "%{prj.name}/src/es_pch.cpp"

  files {
    "%{prj.name}/src/**.h",
    "%{prj.name}/src/**.cpp"
  }

  includedirs {
    "%{prj.name}/src",
		"etherSpace/Vendor/glfw//include"
  }

	links {
		"Glfw",
		"opengl32.lib"
	}

  filter "system:windows"
    cppdialect "C++17"
    staticruntime "On"
    systemversion "latest"

    defines {
      "ES_PLATFORM_WINDOWS",
      "ES_BUILD_DLL"
    }

    postbuildcommands {
      ("{COPY} %{cfg.buildtarget.relpath} ../bin/" .. outputDir .. "/Sandbox")
    }
  
  filter "configurations:Debug"
    defines "ES_DEBUG"
    symbols "On"
  
  filter "configurations:Release"
    defines "ES_RELEASE"
    optimize "On"

  filter "configurations:Dist"
    defines "ES_DIST"
    optimize "On"
