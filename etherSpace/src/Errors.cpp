#include "Errors.h"

Errors::Errors() {
	this->error_type = eErrorType_t::NONE;
	this->ignore_consecutive_errors_with_same_code = true;
}

Errors* Errors::getInstance() {
	if (instance_ptr == nullptr) {
		instance_ptr = new Errors();
	}
	return instance_ptr;
}
void Errors::sendError(eErrorType_t error_type, std::string error_message) {
	this->error_message = error_message;
	this->error_type = error_type;
}
std::pair<eErrorType_t, std::string> Errors::getError() {
	return std::make_pair(this->error_type, this->error_message);
}
void Errors::ignoreConsecutiveErrorsWithSameCode(bool value) {
	this->ignore_consecutive_errors_with_same_code = value;
}