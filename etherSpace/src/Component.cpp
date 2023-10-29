#include "Component.h"

using namespace Components;

Component::Component() {
	this->is_null = true;
}
void Component::start() {}
void Component::update() {}
std::string Component::toString() {
	return "None";
}

Transform::Transform(v2 position, v2 rotation) : Component() {
	this->position = position;
	this->rotation = rotation;
}
void Transform::start() {}
void Transform::update() {}
std::string Transform::toString() {
	return "Transform";
}

Renderer::Renderer(Transform& transform, Shapes::Shape shape, Color color)
	: shape(shape), transform(transform) {
	this->color = color;
}