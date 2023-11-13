#include "Shape.h"

using namespace etherSpace::Shapes;

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

Rectangle::Rectangle(float width, float height, Components::Transform* transform, uint8_t* pixels) {
	this->transform = transform;
	this->pixels = pixels;
	this->width = width;
	this->height = height;
}

void Rectangle::draw() {

}
