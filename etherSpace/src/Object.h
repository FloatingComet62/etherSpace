#pragma once

#include <vector>

#include "Component.h"

class Object {
	bool is_null; // used for invalid object

public:
	std::vector<Components::Component> components;
	Object();
	Object(Components::Transform transform);
	Object(Components::Transform transform, Components::Renderer renderer);
	void addComponent(Components::Component component);
	void addComponents(std::vector<Components::Component> components);
	std::pair<Components::Component, bool> getComponent(std::string compoonentName);
};
