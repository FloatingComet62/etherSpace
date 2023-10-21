#include "Object.h"

Object::Object() {}
Object::Object(Components::Transform transform) {
	this->components.push_back(transform);
}
Object::Object(Components::Transform transform, Components::Renderer renderer) {
	this->components.push_back(transform);
	this->components.push_back(renderer);
}
void Object::addComponent(Components::Component component) {
	this->components.push_back(component);
}
void Object::addComponents(std::vector<Components::Component> components) {
	for (auto& component : components) {
		this->components.push_back(component);
	}
}
/*
Optional<Components::Component> Object::getComponent(std::string compoonentName) {
	for (auto& component : this->components)
		if (component.toString() == compoonentName)
			return Optional<Components::Component>(&component);
	return Optional<Components::Component>();
}
*/