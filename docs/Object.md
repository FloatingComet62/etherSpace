---
updated: 2023-10-18
---

# Properties
* *std::vector\<[Component](Components/README)\>* `components` - The list of components which the object has

# Initialization
Create an empty object
```cpp
Object()
```

Create an object with transform
```cpp
Object(Transform transform)
```

Create an object with transform and renderer
```cpp
Object(Transform transform, Renderer renderer)
```

# Methods
Add a component
```cpp
void addComponent(Component component)
```

Add a list of components
```cpp
void addComponents(std::vector<Component> component)
```

Get a component
```cpp
std::optional<Component> getComponent(std::string componentName)
```

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