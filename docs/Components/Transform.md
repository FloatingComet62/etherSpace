---
tags:
  - component
updated:
  - 2023-10-13
---

# Properties
* *[v2](Vector.md#Vector2)* `position` - Position of the object
* *[v2](Vector.md#Vector2)* `rotation` - Rotation of the object

## Inherited
* *[Object&](Object)* `object` - The object which the component belongs to

# Initialization
Create a Transform with an object, position and rotation
```cpp
Transform(Object& object, v2 position = v2(), v2 rotation = v2())
```

# Methods

## Inherited
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
