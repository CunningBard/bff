# BFO: BFF's Output file
## Version 0.1
### Details
This version is 32 bit, and instructions is only 1 byte, this doesn't support direct string integration. 

### Byte Information
The first 8 bytes is for versioning, 
2 bytes for major, 2 bytes for minor, 2 bytes for incremental, 2 bytes are reserved.
The following bytes are the program instructions, which are only 8 bytes big.

### Argument Handling
Argument to functions are pushed into the stack, 
since the address is pushed into a different stack, 
meaning the address won't interfere.

## Version 0.2
### Details
Same as version 0.1, but now supports direct string integration,

### Byte Information
After the first 8 bytes, the next 8 bytes are the number of strings.
the next 4 bytes is the string length, the next 4 bytes is the address where you want to store the string, 
and the next bytes are the string itself. After the string, the next bytes are the program instructions.