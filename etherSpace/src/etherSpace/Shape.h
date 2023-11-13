#pragma once

#include <stdint.h>

namespace etherSpace {
	namespace Components {
		class Component;
		class Transform;
		class Renderer;
	};

	namespace Shapes {
		class Config {
		public:
			Config(Config const*) = delete;
			void operator=(Config const*) = delete;
			static Config* getConfig();
			void setCenterMode(bool value);
		private:
			bool centerMode;

			static Config* instance_ptr;
			Config();
		};

		class Shape {
		public:
			virtual void draw() = 0;
		protected:
			Components::Transform* transform;
			uint8_t* pixels;
		};

		class Rectangle : public Shape {
		public:
			float width;
			float height;

			Rectangle(float width, float height, Components::Transform* transform, uint8_t* pixels);
			void draw() override;
		};
	};
};
