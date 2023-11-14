#include "es_pch.h"
#include "Component.h"

#include "Object.h"
#include "ErrorManager.h"

using namespace etherSpace::Components;

std::string signatureToString(ComponentSignature componentSignature) {
	switch (componentSignature) {
	case ComponentSignature::TRANSFORM:
		return "Transform";
	case ComponentSignature::RENDERER:
		return "Renderer";
	default:
		return "None";
	};
}

Transform::Transform(Object* object, v2 position, v2 rotation) {
	this->object = object;
	this->position = position;
	this->rotation = rotation;
}

void Transform::start() {}

void Transform::update() {}

ComponentSignature Transform::signature() {
	return ComponentSignature::TRANSFORM;
}

Renderer::Renderer(Object* object, Shapes::Shape* shape, Color color)
	: shape(shape), transform(nullptr) {
	this->object = object;
	this->color = color;
}

void Renderer::start() {
	auto res = this->object->getComponent(ComponentSignature::TRANSFORM);
	if (!res.second) {
		ErrorManager::getInstance()->sendError(eErrorType_t::MISSING_REQUIRED_COMPONENT, "Renderer requires Transform");
		return;
	}
	this->transform = (Components::Transform*)res.first;
}

void Renderer::update() {}

ComponentSignature Renderer::signature() {
	return ComponentSignature::RENDERER;
}