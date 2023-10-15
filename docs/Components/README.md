---
tags:
  - component
updated:
  - 2023-10-13
---

Components provide a way to introduce functionality to objects and interactions they face.

# Properties
* *[Object&](Object)* `object` - The object which the component belongs to

# Initialization
Create a Component with an object
```cpp
Component(Object& object)
```
# Methods
Called once at the start of the program
```cpp
void start()
```

Called every frame
```cpp
void update()
```

Convert to string
```cpp
std::string to_string()
```

# Inbuilt components
- [[Transform]]
- [[Renderer]]
- [[Physics]]