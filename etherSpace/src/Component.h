#pragma once

#include <string>

#include "Vector.h"
#include "Shape.h"
#include "Color.h"

class Object;

namespace Components {
	class Component {
	private:
		Object& object;
		
	public:
		Component(Object& object);
		virtual void start();
		virtual void update();
		virtual std::string toString();
	};
	class Transform : public Component {
	public:
		v2 position;
		v2 rotation;

		Transform(Object& object, v2 position = v2(), v2 rotation = v2());
		void start();
		void update();
		std::string toString();
	};
	class Renderer : public Component {
		Shapes::Shape shape;
		Color color;

		Renderer(Object& object, Shapes::Shape shape, Color color);
	};
};
