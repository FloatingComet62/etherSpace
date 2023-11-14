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
  objdir ("bin-int/" .. outputDir .. "/%{prj.name}")

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

project "EtherSpace"
  location "etherSpace"
  kind "SharedLib"
  language "C++"

  targetdir ("bin/" .. outputDir .. "/%{prj.name}")
  objdir ("bin-int/" .. outputDir .. "/%{prj.name}")

  files {
    "%{prj.name}/src/**.h",
    "%{prj.name}/src/**.cpp"
  }

  -- includedirs {
  --   ""
  -- }

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
