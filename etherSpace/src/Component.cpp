#include "Component.h"

using namespace Components;

Component::Component(Object& object) : object(object) {}
void Component::start() {}
void Component::update() {}
std::string Component::toString() {
	return "None";
}

Transform::Transform(Object& object, v2 position, v2 rotation) : Component(object) {
	this->position = position;
	this->rotation = rotation;
}
void Transform::start() {}
void Transform::update() {}
std::string Transform::toString() {
	return "Transform";
}

Renderer::Renderer(Object& object, Shapes::Shape shape, Color color)
	: Component(object),
		shape(shape) {
	this->color = color;
}