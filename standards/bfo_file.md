# BFO: BFF's Output file
## Version 0.1
### Details
This version is 32 bit, and instructions is only 1 byte, this doesn't support direct string integration. 

### Byte Information
The first 8 bytes is for versioning, 
2 bytes for major, 2 bytes for minor, 2 bytes for incremental, 2 bytes are reserved.
The following bytes are the program instructions, which are only 8 bytes big.

