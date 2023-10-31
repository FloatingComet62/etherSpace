#pragma once

#include <string>

#include "Vector.h"
#include "Shape.h"
#include "Color.h"

class Object;

namespace Components {
	typedef enum {
		TRANSFORM,
		RENDERER
	} ComponentSignature;

	std::string signatureToString(ComponentSignature componentSignature);
	
	class Component {
	protected:
		Object* object;
	public:
		// Pure Virtual Functions - https://www.youtube.com/watch?v=UWAdd13EfM8
		virtual void start() = 0;
		virtual void update() = 0;
		virtual ComponentSignature signature() = 0;
	};

	class Transform : public Component {
	public:
		v2 position;
		v2 rotation;

		Transform(Object* object, v2 position = v2(), v2 rotation = v2());
		void start() override;
		void update() override;
		ComponentSignature signature() override;
	};

	class Renderer : public Component {
		Transform* transform;
		Shapes::Shape shape;
		Color color;
	public:
		Renderer(Object* object, Shapes::Shape shape, Color color);
		void start() override;
		void update() override;
		ComponentSignature signature() override;
	};
};
