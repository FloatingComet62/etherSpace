#include "Object.h"

Object::Object() {
	this->is_null = true;
}
Object::Object(Components::Transform transform) {
	this->is_null = false;
	this->components.push_back(transform);
}
Object::Object(Components::Transform transform, Components::Renderer renderer) {
	this->is_null = false;
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
std::pair<Components::Component, bool> Object::getComponent(std::string compoonentName) {
	for (auto& component : this->components)
		if (component.toString() == compoonentName)
			return std::make_pair(component, true);
	return std::make_pair(Components::Component{ true }, false);
}