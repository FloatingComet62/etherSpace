---
tags:
  - component
updated: 2023-10-15
---

> [!attention] Requires [Transform](/Components/Transform)
# Properties
* *[README](Shape/README.md)* `shape` - Shape of the object
* *[Color](Color.md)* `color` - Color of the object

## Inherited
* *[Object&](Object)* `object` - The object which the component belongs to

# Initialization
Create a Transform with an object, position and rotation
```cpp
Render(Object& object, Shape shape, Color color)
```

# Methods

## Inherited
Called once at the start of the program ^MissingRequiredComponent
> [!danger] Exception: [MISSING_REQUIRED_COMPONENT](/Errors##Missing%20Required%20Component)
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