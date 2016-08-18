#include <string>
#include <stack>
#include <sstream>
#include "types.h"

// Readfile definitions

void matransform (std::stack<mat4> &transfstack, GLfloat * values) ;
void rightmultiply (const mat4 & M, std::stack<mat4> &transfstack) ;
bool readvals (std::stringstream &s, const int numvals, GLfloat * values) ;
void readfile (const char * filename) ;
void parseLine(const std::string &str, std::stack<mat4> &transfstack);
