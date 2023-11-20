#pragma once

#include "../Core.h"

#define NUM_OF_CORE_UNITS 7

namespace etherSpace {
	namespace units {
		class ES_API unit {
		public:
			float value;
			float powers[NUM_OF_CORE_UNITS];
			unit();
			unit(float value, unit unit);
			unit(float value, float* powers);
			~unit();
			unit setValue(float value);
			unit setMeter(float power);
			unit setKilogram(float power);
			unit setSecond(float power);
			unit setAmpere(float power);
			unit setKelvin(float power);
			unit setMole(float power);
			unit setCandela(float power);

			float unit::getPowerDiff(unit const& other) const;
			bool hasSameUnit(unit const& other) const;
			std::string toString() const;
			bool operator==(const unit& val2);
			unit operator+(const unit& val2);
			unit operator-(const unit& val2);
			unit operator*(const unit& val2);
			unit operator/(const unit& val2);
			unit operator*(const float& val2);
			unit operator/(const float& val2);
		};

		/*
		bool operator==(const unit& val1, const unit& val2);
		unit operator+(const unit& val1, const unit& val2);
		unit operator-(const unit& val1, const unit& val2);
		unit operator*(const unit& val1, const unit& val2);
		unit operator/(const unit& val1, const unit& val2);
		unit operator*(const unit& val1, const float& val2);
		unit operator/(const unit& val1, const float& val2);
		unit operator*(const float& val1, const unit & val2);
		unit operator/(const float& val1, const unit& val2);
		*/
	}
}
