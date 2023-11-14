#pragma once

namespace etherSpace {
	class v2 {
	public:
		float i;
		float j;

		v2(float i = 0, float j = 0);
		std::string toString() const;
	};

	class v3 {
	public:
		float i;
		float j;
		float k;

		v3(float i = 0, float j = 0, float k = 0);
		v3(v2 vec);
		std::string toString() const;
	};

	float dot(const v3& vec1, const v3& vec2);
	v3 cross(const v3& vec1, const v3& vec2);
};
