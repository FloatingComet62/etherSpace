---
updated: 2023-10-15
---

Represents Color in etherSpace

# Properties
* *uint8_t* `red` - Amount of Red Color
* *uint8_t* `green` - Amount of Green Color
* *uint8_t* `blue` - Amount of Blue Color
* *uint8_t* `alpha` - Amount of Alpha Color

# Initialization
Initialize the color with RGBA values
```cpp
Color(uint8_t red = 0, uint8_t green = 0, uint8_t blue = 0, uint8_t alpha = 0)
```

Initialize the color with hex string ^InvalidHexString
> [!danger] Exception: [INVALID_HEX_STRING](ErrorManager.md##Invalid%20Hex%20String)
```cpp
Color(std::string hex_string)
```

# Methods
Convert to RGBA array
```cpp
std::array<uint8_t, 4> toRGBA()
```

Convert to BGRA array
```cpp
std::array<uint8_t, 4> toBGRA()
```

Convert to hex string
```cpp
std::string toString()
```
