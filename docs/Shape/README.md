---
tags:
  - shape
updated: 2023-10-18
---
Represents a shape in etherSpace

# List of Shapes
* `RECTANGLE` - Represents a rectangle
* `CIRCLE` - Represents a circle
* `TRIANGLE` - Represents a triangle

# Shape Single Config
*bool* `centerMode` - If true, the object's transform will represent the geometric center. If false, the object's transform will represent the top left corner.
To modify this property
```cpp
Shape::Config::getConfig().setCenterMode(/* new value */);
```

# Shape
## Properties
* *[Transform&](Components/Transform)* `transform` - Transform of the object to render
* *uint8_t\*&* `pixels` - The pixel's buffer to manipulate

## Methods
Update the pixel's buffer with the rendering of the object
```cpp
void draw();
```