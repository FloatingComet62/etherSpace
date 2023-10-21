#pragma once

#include <vector>

#include "Component.h"
#include "Optional.h"

class Object {
	std::vector<Components::Component> components;

public:
	Object();
	Object(Components::Transform transform);
	Object(Components::Transform transform, Components::Renderer renderer);
	void addComponent(Components::Component component);
	void addComponents(std::vector<Components::Component> components);
	// Optional<Components::Component> getComponent(std::string compoonentName);
};
