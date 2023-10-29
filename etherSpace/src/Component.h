#pragma once

#include <string>

#include "Vector.h"
#include "Shape.h"
#include "Color.h"

namespace Components {
	class Component {
	bool is_null; // used for invalid component
		
	public:
		Component();
		virtual void start();
		virtual void update();
		virtual std::string toString();
	};

	class Transform : public Component {
	public:
		v2 position;
		v2 rotation;

		Transform(v2 position = v2(), v2 rotation = v2());
		void start();
		void update();
		std::string toString();
	};

	class Renderer : public Component {
		Shapes::Shape shape;
		Color color;
		Transform& transform;

		Renderer(Transform& transform, Shapes::Shape shape, Color color);
	};
};
