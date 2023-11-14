#include "es_pch.h"
#include "Object.h"

using namespace etherSpace;

Object::Object() {}

void Object::addComponent(Components::Component* component) {
	this->components.push_back(component);
}

void Object::addComponents(std::vector<Components::Component*> components) {
	for (auto& component : components) {
		this->components.push_back(component);
	}
}

std::pair<Components::Component*, bool> Object::getComponent(Components::ComponentSignature componentSignature) {
	for (auto& component : this->components) {
		if (component->signature() == componentSignature) {
			return std::make_pair(component, true);
		}
	}
	return std::make_pair(nullptr, false);
}