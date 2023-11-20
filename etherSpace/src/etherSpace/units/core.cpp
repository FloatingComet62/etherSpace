#include "es_pch.h"
#include "core.h"

#include "../modules/Log.h"
#include <math.h>

using namespace etherSpace::units;

static unsigned int meter = 0;
static unsigned int kilogram = 1;
static unsigned int second = 2;
static unsigned int ampere = 3;
static unsigned int kelvin = 4;
static unsigned int mole = 5;
static unsigned int candela = 6;

static std::string getStringOfCoreUnit(unsigned int i) {
	switch (i) {
	case 0:
		return "m";
	case 1:
		return "kg";
	case 2:
		return "s";
	case 3:
		return "A";
	case 4:
		return "K";
	case 5:
		return "mol";
	case 6:
		return "cd";
	default:
		return "x";
	}
}

unit::unit() : powers() {
	this->value = 0;
}
unit::unit(float value, unit unit) {
	this->value = value;
	std::copy(
		std::begin(unit.powers),
		std::end(unit.powers),
		std::begin(this->powers)
	);
}
unit::unit(float value, float* powers) {
	this->value = value;
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		this->powers[i] = powers[i];
	}
}
unit::~unit() {}
unit unit::setValue(float value) {
	this->value = value;
	return *this;
}
unit unit::setMeter(float power) {
	this->powers[meter] = power;
	return *this;
}
unit unit::setKilogram(float power) {
	this->powers[kilogram] = power;
	return *this;
}
unit unit::setSecond(float power) {
	this->powers[second] = power;
	return *this;
}
unit unit::setAmpere(float power) {
	this->powers[ampere] = power;
	return *this;
}
unit unit::setKelvin(float power) {
	this->powers[kelvin] = power;
	return *this;
}
unit unit::setMole(float power) {
	this->powers[mole] = power;
	return *this;
}
unit unit::setCandela(float power) {
	this->powers[candela] = power;
	return *this;
}

bool unit::hasSameUnit(const unit& val2) const {
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		float power1 = this->powers[i];
		float power2 = val2.powers[i];
		if ((power1 == 0 && power2 != 0) || (power1 != 0 && power2 == 0)) {
			return false;
		}
	}
	return true;
}

float unit::getPowerDiff(const unit& val2) const {
	float powerSum1 = 0;
	for (auto& power : this->powers) {
		powerSum1 += power;
	}
	float powerSum2 = 0;
	for (auto& power : val2.powers) {
		powerSum2 += power;
	}
	return powerSum1 - powerSum2;
}

std::string unit::toString() const {
	std::string str = std::to_string(this->value);
	str.erase(str.find_last_not_of('0') + 1, std::string::npos);
	str.erase(str.find_last_not_of('.') + 1, std::string::npos);
	str += " ";
	// +ve powers first
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		auto power = this->powers[i];
		if (power <= 0) {
			continue;
		}
		std::string label = getStringOfCoreUnit(i);
		if (power == 1) {
			str += label;
			continue;
		}
		std::string power_str = std::to_string(power);
		power_str.erase(power_str.find_last_not_of('0') + 1, std::string::npos);
		power_str.erase(power_str.find_last_not_of('.') + 1, std::string::npos);
		str += label + "^" + power_str;
	}
	// -ve powers next
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		auto power = this->powers[i];
		if (power >= 0) {
			continue;
		}
		std::string label = getStringOfCoreUnit(i);
		std::string power_str = std::to_string(power);
		power_str.erase(power_str.find_last_not_of('0') + 1, std::string::npos);
		power_str.erase(power_str.find_last_not_of('.') + 1, std::string::npos);
		str += label + "^" + power_str;
	}
	return str;
}

bool unit::operator==(const unit& val2) {
	return this->hasSameUnit(val2) &&
		this->value == val2.value;
}
unit unit::operator+(const unit& val2) {
	if (!this->hasSameUnit(val2)) {
		etherSpace::Log::critical("Can't add 2 different units");
	}
	auto diff = this->getPowerDiff(val2);
	return unit(this->value + val2.value * pow(10.0f, diff), *this);
}

unit unit::operator-(const unit& val2) {
	if (!this->hasSameUnit(val2)) {
		etherSpace::Log::critical("Can't subtract 2 different units");
	}
	auto diff = this->getPowerDiff(val2);
	return unit(this->value - val2.value * pow(10.0f, diff), *this);
}
unit unit::operator*(const unit& val2) {
	float powerSums[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerSums[i] = this->powers[i] + val2.powers[i];
	}
	return unit(this->value * val2.value, powerSums);
}
unit unit::operator/(const unit& val2) {
	float powerDiff[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerDiff[i] = this->powers[i] - val2.powers[i];
	}
	return unit(this->value * val2.value, powerDiff);
}
unit unit::operator*(const float& val2) {
	return unit(val2 * this->value, *this);
}
unit unit::operator/(const float& val2) {
	float powerNeg[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerNeg[i] = -this->powers[i];
	}
	return unit(this->value / val2, powerNeg);
}
/*
bool etherSpace::units::operator==(const unit& val1, const unit& val2) {
	return val1.hasSameUnit(val2) &&
		val1.value == val2.value;
}
unit etherSpace::units::operator+(const unit& val1, const unit& val2) {
	if (!val1.hasSameUnit(val2)) {
		etherSpace::Log::critical("Can't add 2 different units");
	}
	auto diff = val1.getPowerDiff(val2);
	return unit(val1.value + val2.value * pow(10.0f, diff), val1);
}

unit etherSpace::units::operator-(const unit& val1, const unit& val2) {
	if (!val1.hasSameUnit(val2)) {
		etherSpace::Log::critical("Can't subtract 2 different units");
	}
	auto diff = val1.getPowerDiff(val2);
	return unit(val1.value - val2.value * pow(10.0f, diff), val1);
}
unit etherSpace::units::operator*(const unit& val1, const unit& val2) {
	float powerSums[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerSums[i] = val1.powers[i] + val2.powers[i];
	}
	return unit(val1.value * val2.value, powerSums);
}
unit etherSpace::units::operator/(const unit& val1, const unit& val2) {
	float powerDiff[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerDiff[i] = val1.powers[i] - val2.powers[i];
	}
	return unit(val1.value * val2.value, powerDiff);
}
unit etherSpace::units::operator*(const unit& val1, const float& val2) {
	return unit(val2 * val1.value, val1);
}
unit etherSpace::units::operator/(const unit& val1, const float& val2) {
	float powerNeg[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerNeg[i] = -val1.powers[i];
	}
	return unit(val1.value / val2, powerNeg);
}
unit etherSpace::units::operator*(const float& val1, const unit& val2) {
	return unit(val1 * val2.value, val2);
}
unit etherSpace::units::operator/(const float& val1, const unit& val2) {
	float powerNeg[NUM_OF_CORE_UNITS] = {};
	for (unsigned int i = 0; i < NUM_OF_CORE_UNITS; i++) {
		powerNeg[i] = -val2.powers[i];
	}
	return unit(val1 / val2.value, powerNeg);
}*/

#undef NUM_OF_CORE_UNITS
