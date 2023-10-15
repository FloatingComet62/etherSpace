---
tags:
  - shape
updated:
  - 2023-10-14
---

Represents a shape in etherSpace

# Properties
* *[Transform&](Components/Transform)* `transform` - Transform of the object to render
* *bool* `centerMode` - If true, the object's transform will represent the geometric center. If false, the object's transform will represent the top left corner
* *uint8_t\*&* `pixels` - The pixel's buffer to manipulate
# Methods
Update the pixel's buffer with the rendering of the object
```cpp
void draw();
```