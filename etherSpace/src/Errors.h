#pragma once

#include <iostream>
#include <string>

typedef enum {
	NONE, TEST, INVALID_HEX_STRING, MISSING_REQUIRED_COMPONENT
} eErrorType_t;

std::string errorTypeToString(eErrorType_t error_type);

class ErrorManager {
private:
	std::string error_message;
	eErrorType_t error_type;
	bool ignore_consecutive_errors_with_same_code;

	static ErrorManager* instance_ptr;
	ErrorManager();
public:
	ErrorManager(ErrorManager const*) = delete;
	void operator=(ErrorManager const*) = delete;
	static ErrorManager* getInstance();
	bool hasError();
	void sendError(eErrorType_t error_type, std::string error_message);
	void clearError();
	std::pair<eErrorType_t, std::string> getError();
	void ignoreConsecutiveErrorsWithSameCode(bool value);
};
