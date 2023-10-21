#pragma once

#include <stdint.h>

namespace Components {
	class Component;
	class Transform;
	class Renderer;
};

namespace Shapes {
	class Config {
	private:
		bool centerMode;

		static Config* instance_ptr;
		Config();
	public:
		Config(Config const*) = delete;
		void operator=(Config const*) = delete;
		static Config* getConfig();
		void setCenterMode(bool value);
	};

	class Shape {
		Components::Transform& transform;
		uint8_t*& pixels;
	public:
		Shape(Components::Transform& transform, uint8_t*& pixels);
		virtual void draw();
	};

	class Rectangle : Shape {
	public:
		float width;
		float height;

		Rectangle(float width, float height, Components::Transform& transform, uint8_t*& pixels);
	};
}
