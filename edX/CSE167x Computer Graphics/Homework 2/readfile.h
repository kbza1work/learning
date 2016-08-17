#include <string>
#include <stack>
#include <sstream>
#include "types.h"

using namespace std;

// Readfile definitions

void matransform (stack<mat4> &transfstack, GLfloat * values) ;
void rightmultiply (const mat4 & M, stack<mat4> &transfstack) ;
bool readvals (stringstream &s, const int numvals, GLfloat * values) ;
void readfile (const char * filename) ;
void parseLine(const string &str, stack<mat4> &transfstack);
