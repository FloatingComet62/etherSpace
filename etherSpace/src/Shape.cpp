#include "Shape.h"

using namespace Shapes;

Config::Config() {
	this->centerMode = true;
}

Config* Config::getConfig() {
	static Config instance_ptr;
	return &instance_ptr;
}

void Config::setCenterMode(bool value) {
	this->centerMode = value;
}

Shape::Shape(Components::Transform& transform, uint8_t*& pixels)
	: transform(transform),
		pixels(pixels) {
}
void Shape::draw() {}

Rectangle::Rectangle(float width, float height, Components::Transform& transform, uint8_t*& pixels)
	: Shape(transform, pixels) {
	this->width = width;
	this->height = height;
}
