#pragma once

#include <string>

typedef enum {
	NONE, TEST, INVALID_HEX_STRING, MISSING_REQUIRED_COMPONENT
} eErrorType_t;

class Errors {
private:
	std::string error_message;
	eErrorType_t error_type;
	static Errors* instance_ptr;
	bool ignore_consecutive_errors_with_same_code;
	Errors();
public:
	Errors(Errors& obj) = delete;
	void operator=(const Errors&) = delete;
	static Errors* getInstance();
	void sendError(eErrorType_t error_type, std::string error_message);
	std::pair<eErrorType_t, std::string> getError();
	void ignoreConsecutiveErrorsWithSameCode(bool value);
};
