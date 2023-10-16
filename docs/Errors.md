---
updated: 2023-10-15
---

# List of Errors
* `TEST` - Used for testing purposes
* `INVALID_HEX_STRING` - When an invalid hex string is passed
* `MISSING_REQUIRED_COMPONENT` - When a component requires another component but it is not present.

# References
## Invalid Hex String
[Color Initializer](/Color#^InvalidHexString)

## Missing Required Component
[Renderer start](/Components/Renderer#^MissingRequiredComponent)

# Error System
`Error` is singleton class which manages error handling in etherSpace

## Methods
Get the singleton instance of Error
```cpp
Errors* getInstance();
```

Send an error to the Error system
```cpp
void sendError(eErrorType_t error_type, std::string error_message);
```

> [!info]
> If 2 consecutive errors of the same code are received, etherSpace will consider that it's the same error, and not display it.
> If you wish the send a error and make sure the is displayed, you can set the 3rd argument to true
> ```cpp
Errors::getInstance().sendError(Errors::TEST, "This is an emergency error message", true);
> ```
> If you wish to turn off this feature all together, run this at the start of the program
> ```cpp
Errors::getInstance().ignoreConsecutiveErrorsWithSameCode(false);
> ```
> You can change this property anytime in the program

Get the error being displayed by the error system
```cpp
std::pair<eErrorType_t, std::string> getError();
```

```cpp
void ignoreConsecutiveErrorsWithSameCode(bool value);
```