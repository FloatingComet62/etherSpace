#pragma once

#include "Component.h"

namespace etherSpace {
	class Object {
	public:
		std::vector<Components::Component*> components;
		Object();
		void addComponent(Components::Component* component);
		void addComponents(std::vector<Components::Component*> components);
		std::pair<Components::Component*, bool> getComponent(Components::ComponentSignature componentSignature);
	};
};
