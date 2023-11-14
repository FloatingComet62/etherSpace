#include "es_pch.h"
#include "ErrorManager.h"

#include "Log.h"

using namespace etherSpace;

std::string etherSpace::errorTypeToString(eErrorType_t error_type) {
	switch (error_type) {
	case eErrorType_t::NONE:
		return "None";
	case eErrorType_t::TEST:
		return "Test";
	case eErrorType_t::INVALID_HEX_STRING:
		return "Invalid Hex String";
	case eErrorType_t::MISSING_REQUIRED_COMPONENT:
		return "Missing Required Component";
	default:
		return "Unknown";
	}
}

ErrorManager::ErrorManager() {
	this->error_type = eErrorType_t::NONE;
	this->error_message = "";
	this->ignore_consecutive_errors_with_same_code = true;
}

ErrorManager* ErrorManager::getInstance() {
	static ErrorManager instance_ptr;
	return &instance_ptr;
}

bool ErrorManager::hasError() {
	return this->error_type != eErrorType_t::NONE;
}

void ErrorManager::sendError(eErrorType_t error_type, const std::string& error_message) {
	if (this->ignore_consecutive_errors_with_same_code && this->error_type == error_type) {
		return;
	}
	this->error_message = error_message;
	this->error_type = error_type;

	std::string message = "[";
	message += errorTypeToString(error_type);
	message += "] ";
	message += error_message;
	Log::critical(message);
}

void ErrorManager::clearError() {
	this->error_type = eErrorType_t::NONE;
	this->error_message = "";
}

std::pair<eErrorType_t, std::string> ErrorManager::getError() {
	return std::make_pair(this->error_type, this->error_message);
}

void ErrorManager::ignoreConsecutiveErrorsWithSameCode(bool value) {
	this->ignore_consecutive_errors_with_same_code = value;
}
