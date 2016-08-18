#include "include/catch.hpp"

#include <string>
#include <stack>
#include "types.h"
#include "variables.h"
#include "readfile.h"

void resetGlobals() {
  eye = eyeinit;
  up = upinit;
  center = vec3(0.0, 0.0, 0.0);
  w = 600;
  h = 400;
  fovy = 90.0;
  useGlu = false;
  sx = 0.0;
  sy = 0.0;
  std::fill_n(lightposn, 4 * numLights, 0.0);
  std::fill_n(lightcolor, 4 * numLights, 0.0);
  std::fill_n(lightransf, 4 * numLights, 0.0);
  numused = 0;
  std::fill_n(ambient, 4, 0.0);
  std::fill_n(diffuse, 4, 0.0);
  std::fill_n(specular, 4, 0.0);
  std::fill_n(emission, 4, 0.0);
  shininess = 0.0;
  numobjects = 0;
}

TEST_CASE("reading light data", "[readfile][light]") {
  resetGlobals();

  SECTION("for a single light") {
    std::stack <mat4> transfstack;
    transfstack.push(mat4(1.0));
    const std::string input = "light 1.0 -2.0 5.0 1.0 1.0 0.83 0.04 1.0";
    parseLine(input, transfstack);

    SECTION("number of lights") {
      REQUIRE(numused == 1);
    }

    SECTION("light positions") {
      const GLfloat expected[4 * numLights] = { 1.0, -2.0, 5.0, 1.0 };
      for(int i = 0; i < (4 * numLights); i++) {
        REQUIRE(lightposn[i] == Approx(expected[i]));
      }
    }

    SECTION("light color") {
      const GLfloat expected[4 * numLights] = { 1.0, 0.83, 0.04, 1.0 };
      for(int i = 0; i < (4 * numLights); i++) {
        REQUIRE(lightcolor[i] == Approx(expected[i]));
      }
    }
  }
}

