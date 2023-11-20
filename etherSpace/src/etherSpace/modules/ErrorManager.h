#pragma once

#include "../Core.h"

namespace etherSpace {
	typedef enum {
		NONE, TEST, INVALID_HEX_STRING, MISSING_REQUIRED_COMPONENT
	} eErrorType_t;

	std::string errorTypeToString(eErrorType_t error_type);

	class ES_API ErrorManager {
	public:
		static ErrorManager* getInstance();
		bool hasError() const;
		void sendError(eErrorType_t error_type, const std::string& error_message);
		void clearError();
		std::pair<eErrorType_t, std::string> getError() const;
		void ignoreConsecutiveErrorsWithSameCode(bool value);
	private:
		ErrorManager(ErrorManager const*) = delete;
		void operator=(ErrorManager const*) = delete;
		std::string error_message;
		eErrorType_t error_type;
		bool ignore_consecutive_errors_with_same_code;

		static ErrorManager* instance_ptr;
		ErrorManager();
	};
};
