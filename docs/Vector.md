---
updated: 2023-10-13
---

# Functions
Calculate the dot product of 2 vectors
```cpp
float dot(v3 vec1, v3 vec2)
```

Calculate the cross product of 2 vectors
```cpp
v3 cross(v3 vec1, v3 vec2)
```

# Vector2
## Properties
* *float* `i` - I component of the vector
* *float* `j` - J component of the vector

## Initialization
Initialize the 2D vector
```cpp
v2(float i = 0, float j = 0)
```

## Methods
Convert to string
```cpp
std::string toString()
```

# Vector3
## Properties
* *float* `i` - I component of the vector
* *float* `j` - J component of the vector
* *float* `k` - K component of the vector

## Initialization
Initialize the 3D vector with i, j and k components
```cpp
v3(float i = 0, float j = 0, float k = 0)
```

Initialize the 3D vector with 2D vector
```cpp
v3(v2 vec)
```

## Methods
Convert to string
```cpp
std::string toString()
```
