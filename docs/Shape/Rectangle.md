---
tags:
  - shape
updated:
  - 2023-10-14
---

Rectangle in etherSpace

# Properties
* *float* width - Width of the rectangle
* *float* height - Height of the rectangle
## Inherited
* *[Transform&](Components/Transform)* `transform` - Transform of the object to render
* *bool* `centerMode` - If true, the object's transform will represent the geometric center. If false, the object's transform will represent the top left corner
* *uint8_t\*&* `pixels` - The pixel's buffer to manipulate

# Initialization
Initialize with width, height, transform, center mode and pixels
```cpp
Rectangle(float width, float height, Transform& transform, bool centerMode, uint8_t*& pixels)
```
# Methods
## Inherited
Update the pixel's buffer with the rendering of the object
```cpp
void draw();
```