#include "debug.h"
#include <iostream>

void DEBUG(const mat4& matrix, const std::string& name) {
  std::cout << name << " = ";
  for(int i = 0; i < 4; i++) {
    if(i > 0) {
      const int numSpaces = name.length() + 3;
      for(int spacesCounter = 0; spacesCounter < numSpaces; spacesCounter++) {
        std::cout << " ";
      }
    }

    std::cout << "( " << matrix[0][i] << ", " << matrix[1][i] << ", " <<
      matrix[2][i] << ", " << matrix[3][i] << " )" << std::endl;
  }
}
