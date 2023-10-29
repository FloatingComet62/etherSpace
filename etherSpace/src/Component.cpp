#include "Component.h"

using namespace Components;

Component::Component(bool is_null = true) {
	this->is_null = is_null;
}
void Component::start() {}
void Component::update() {}
std::string Component::toString() {
	return "None";
}

Transform::Transform(v2 position, v2 rotation) : Component(false) {
	this->position = position;
	this->rotation = rotation;
}
void Transform::start() {}
void Transform::update() {}
std::string Transform::toString() {
	return "Transform";
}

Renderer::Renderer(Transform* transform, Shapes::Shape shape, Color color)
	: Component(false), shape(shape), transform(transform) {
	this->color = color;
}
std::string Renderer::toString() {
	return "Renderer";
}