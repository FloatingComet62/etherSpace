#include "Vector.h"

v2::v2(float i, float j) {
	this->i = i;
	this->j = j;
}

std::string v2::toString() {
	return "v2 { i: "
		+ std::to_string(this->i)
		+ ", j: " + std::to_string(this->j)
		+ " }";
}

v3::v3(float i, float j, float k) {
	this->i = i;
	this->j = j;
	this->k = k;
}

v3::v3(v2 vec) {
	this->i = vec.i;
	this->j = vec.j;
	this->k = 0;
}

std::string v3::toString() {
	return "v3 { i: "
		+ std::to_string(this->i)
		+ ", j: " + std::to_string(this->j)
		+ ", k: " + std::to_string(this->k)
		+ " }";
}

float dot(v3 vec1, v3 vec2) {
	return vec1.i * vec2.i + vec1.j * vec2.j + vec1.k * vec2.k;
}
v3 cross(v3 vec1, v3 vec2) {
	return v3(
		vec1.j * vec2.k - vec1.k * vec2.j,
	  - vec1.i * vec2.k + vec1.k * vec2.i,
		vec1.i * vec2.j - vec1.j * vec2.i
	);
}

