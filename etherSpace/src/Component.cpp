#include "Component.h"

Components::Component::Component(Object& object) : object(object) {}
void Components::Component::start() {}
void Components::Component::update() {}
std::string Components::Component::toString() {
	return "None";
}

Components::Transform::Transform(Object& object, v2 position, v2 rotation) : Component(object) {
	this->position = position;
	this->rotation = rotation;
}
void Components::Transform::start() {}
void Components::Transform::update() {}
std::string Components::Transform::toString() {
	return "Transform";
}

Components::Renderer::Renderer(Object& object, Shapes::Shape shape, Color color) : Component(object) {
	this->color = color;
}