#include "Color.h"

#include <iostream>
#include "Errors.h"

Color::Color(uint8_t red, uint8_t green, uint8_t blue , uint8_t alpha) {
	this->red = red;
	this->green = green;
	this->blue = blue;
	this->alpha = alpha;
}

uint8_t hexCharToInt(char c) {
	if ((c >= '0') && (c <= '9')) return c - '0';
	if ((c >= 'A') && (c <= 'F')) return 10 + c - 'A';
	if ((c >= 'a') && (c <= 'f')) return 10 + c - 'a';
	return 'q';
}

// examples i wrote before i coded the function below
// 17 -> 1 * 16 + 7 = 23
// a8 -> 10 * 16 + 8 = 168
uint8_t hexStrToValue(std::string str) {
	return hexCharToInt(str[0]) * 16 + hexCharToInt(str[1]);
}

char intTohexChar(uint8_t integer) {
    if (integer < 10) return '0' + integer;
    else return 'A' + integer - 10;
}

// examples i wrote before i coded the function below
// 23 = 1 * 16 + 7 -> 17
// 168 -> 10 * 16 + 8 = a8
std::string valueToHexStr(uint8_t hex) {
    auto div_mod = std::div(hex, 16);
    char hex_string[3] = { intTohexChar(div_mod.quot), intTohexChar(div_mod.rem), '\0' };
    std::string s(hex_string, 3);
    return s;
}

Color::Color(std::string hex_string) {
    if (hex_string[0] == '#') hex_string.erase(0, 1);

    std::string parsed_string;
    {
        size_t hex_len = hex_string.length();
        switch (hex_len) {
        case 3:
        case 4:
            for (auto& character : hex_string) {
                parsed_string += character;
                parsed_string += character;
            }
            break;
        case 6:
        case 8:
            parsed_string += hex_string;
            break;

        default:
            break;
        }
        if (hex_len == 3 || hex_len == 6) {
            parsed_string += "ff";  // for alpha
        }
    }

    if (parsed_string.length() != 8) {
        // some weird invalid hex was provided
        std::string errorMessage = hex_string;
        errorMessage += " is not a valid hex string\n";
        ErrorManager::getInstance()->sendError(eErrorType_t::INVALID_HEX_STRING,
            errorMessage);
        this->red = 0;
        this->green = 0;
        this->blue = 0;
        this->alpha = 0;
        return;
    }

    this->red = hexStrToValue(parsed_string.substr(0, 2));
    this->green = hexStrToValue(parsed_string.substr(2, 2));
    this->blue = hexStrToValue(parsed_string.substr(4, 2));
    this->alpha = hexStrToValue(parsed_string.substr(6, 2));
}

std::array<uint8_t, 4> Color::toRGBA() {
    std::array<uint8_t, 4> t = {this->red, this->green, this->blue, this->alpha};
    return t;
}
std::array<uint8_t, 4> Color::toBGRA() {
    std::array<uint8_t, 4> t = {this->blue, this->green, this->red, this->alpha};
    return t;
}

std::string Color::toString() {
    std::string hex_string = "#";
    hex_string += valueToHexStr(this->red);
    hex_string += valueToHexStr(this->green);
    hex_string += valueToHexStr(this->blue);
    hex_string += valueToHexStr(this->alpha);
    return hex_string;
}