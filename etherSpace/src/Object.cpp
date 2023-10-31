#include <iostream>

#include "Object.h"

Object::Object() {}
void Object::addComponent(Components::Component* component) {
	this->components.push_back(component);
}
void Object::addComponents(std::vector<Components::Component*> components) {
	for (auto& component : components) {
		this->components.push_back(component);
	}
}
std::pair<Components::Component*, bool> Object::getComponent(std::string componentName) {
	for (auto& component : this->components) {
		if (component->toString() == componentName) {
			return std::make_pair(component, true);
		}
	}
	return std::make_pair(&Components::Component{ true }, false);
}