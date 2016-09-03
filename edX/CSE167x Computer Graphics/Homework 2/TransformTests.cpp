#include "include/catch.hpp"

#include "Transform.h"
#include <glm/glm.hpp>
#include <glm/gtc/type_ptr.hpp>

TEST_CASE("Transform::rotate()", "[transform][rotate]") {
  SECTION("by a positive angle around the y-axis") {
    const mat4 rotation = Transform::rotate(90.0, vec3(0.0, 2.0, 0.0));
    const vec4 result =  rotation * vec4(1.0, 1.0, 1.0, 1.0);
    REQUIRE(result.x == Approx(1.0));
    REQUIRE(result.y == Approx(1.0));
    REQUIRE(result.z == Approx(-1.0));
    REQUIRE(result.w == Approx(1.0));
  }

  SECTION("by a negative angle around the z-axis") {
    const mat4 rotation = Transform::rotate(-90.0, vec3(0.0, 0.0, 2.0));
    const vec4 result =  rotation * vec4(1.0, 1.0, 1.0, 1.0);
    REQUIRE(result.x == Approx(1.0));
    REQUIRE(result.y == Approx(-1.0));
    REQUIRE(result.z == Approx(1.0));
    REQUIRE(result.w == Approx(1.0));
  }

  SECTION("by a positive canted angle") {
    const mat4 rotation = Transform::rotate(90.0, vec3(-1.0, 1.0, -1.0));
    const vec4 result =  rotation * vec4(1.0, 1.0, -1.0, 1.0);
    REQUIRE(result.x == Approx(-0.333333333));
    REQUIRE(result.y == Approx(-0.821367205));
    REQUIRE(result.z == Approx(-1.488033872));
    REQUIRE(result.w == Approx(1.0));
  }

  SECTION("around an axis parallel to the original vector") {
    const mat4 rotation = Transform::rotate(-60.0, vec3(-1.0, 4.0, 6.3));
    const vec4 result =  rotation * vec4(-2.0, 8.0, 12.6, 1.0);
    REQUIRE(result.x == Approx(-2.0));
    REQUIRE(result.y == Approx(8.0));
    REQUIRE(result.z == Approx(12.6));
    REQUIRE(result.w == Approx(1.0));
  }
}

TEST_CASE("Transform::left()", "[transform][left]") {
  SECTION("by a positive angle around the y-axis") {
    const float angle = 90.0;
    vec3 eye = vec3(0.0, 0.0, 5.0);
    vec3 up = vec3(0.0, 5.0, 0.0);
    Transform::left(angle, eye, up);

    SECTION("eye x-component") {
      REQUIRE(eye.x == Approx(5.0));
    }
    SECTION("eye y-component") {
      REQUIRE(eye.y == Approx(0.0));
    }
    SECTION("eye z-component") {
      REQUIRE(eye.z == Approx(0.0));
    }

    SECTION("up x-component") {
      REQUIRE(up.x == Approx(0.0));
    }
    SECTION("up y-component") {
      REQUIRE(up.y == Approx(5.0));
    }
    SECTION("up z-component") {
      REQUIRE(up.z == Approx(0.0));
    }
  }

  SECTION("by a negative angle around the y-axis") {
    const float angle = -90.0;
    vec3 eye = vec3(0.0, 0.0, 5.0);
    vec3 up = vec3(0.0, 5.0, 0.0);
    Transform::left(angle, eye, up);

    SECTION("eye x-component") {
      REQUIRE(eye.x == Approx(-5.0));
    }
    SECTION("eye y-component") {
      REQUIRE(eye.y == Approx(0.0));
    }
    SECTION("eye z-component") {
      REQUIRE(eye.z == Approx(0.0));
    }

    SECTION("up x-component") {
      REQUIRE(up.x == Approx(0.0));
    }
    SECTION("up y-component") {
      REQUIRE(up.y == Approx(5.0));
    }
    SECTION("up z-component") {
      REQUIRE(up.z == Approx(0.0));
    }
  }

  SECTION("by a canted positive angle") {
    const float angle = 90.0;
    vec3 eye = vec3(0.0, 0.0, 5.0);
    vec3 up = vec3(0.5, 0.5, 0.0);
    Transform::left(angle, eye, up);

    SECTION("eye x-component") {
      REQUIRE(eye.x == Approx(3.535533906));
    }
    SECTION("eye y-component") {
      REQUIRE(eye.y == Approx(-3.535533906));
    }
    SECTION("eye z-component") {
      REQUIRE(eye.z == Approx(0.0));
    }

    SECTION("up x-component") {
      REQUIRE(up.x == Approx(0.5));
    }
    SECTION("up y-component") {
      REQUIRE(up.y == Approx(0.5));
    }
    SECTION("up z-component") {
      REQUIRE(up.z == Approx(0.0));
    }
  }
}

TEST_CASE("Transform::up()", "[transform][up]") {
  SECTION("by a positive angle around the y-axis") {
    const float angle = 90.0;
    vec3 eye = vec3(0.0, 0.0, 5.0);
    vec3 up = vec3(0.0, 5.0, 0.0);
    Transform::up(angle, eye, up);

    SECTION("eye x-component") {
      REQUIRE(eye.x == Approx(0.0));
    }
    SECTION("eye y-component") {
      REQUIRE(eye.y == Approx(5.0));
    }
    SECTION("eye z-component") {
      REQUIRE(eye.z == Approx(0.0));
    }

    SECTION("up x-component") {
      REQUIRE(up.x == Approx(0.0));
    }
    SECTION("up y-component") {
      REQUIRE(up.y == Approx(0.0));
    }
    SECTION("up z-component") {
      REQUIRE(up.z == Approx(-5.0));
    }
  }

  SECTION("by a negative angle around the y-axis") {
    const float angle = -90.0;
    vec3 eye = vec3(0.0, 0.0, 5.0);
    vec3 up = vec3(0.0, 5.0, 0.0);
    Transform::up(angle, eye, up);

    SECTION("eye x-component") {
      REQUIRE(eye.x == Approx(0.0));
    }
    SECTION("eye y-component") {
      REQUIRE(eye.y == Approx(-5.0));
    }
    SECTION("eye z-component") {
      REQUIRE(eye.z == Approx(0.0));
    }

    SECTION("up x-component") {
      REQUIRE(up.x == Approx(0.0));
    }
    SECTION("up y-component") {
      REQUIRE(up.y == Approx(0.0));
    }
    SECTION("up z-component") {
      REQUIRE(up.z == Approx(5.0));
    }
  }

  SECTION("by a canted positive angle") {
    const float angle = 90.0;
    vec3 eye = vec3(5.0, 5.0, 0.0);
    vec3 up = vec3(-0.5, 0.5, 0.0);
    Transform::up(angle, eye, up);

    SECTION("eye x-component") {
      REQUIRE(eye.x == Approx(-5.0));
    }
    SECTION("eye y-component") {
      REQUIRE(eye.y == Approx(5.0));
    }
    SECTION("eye z-component") {
      REQUIRE(eye.z == Approx(0.0));
    }

    SECTION("up x-component") {
      REQUIRE(up.x == Approx(-0.5));
    }
    SECTION("up y-component") {
      REQUIRE(up.y == Approx(-0.5));
    }
    SECTION("up z-component") {
      REQUIRE(up.z == Approx(0.0));
    }
  }
}

TEST_CASE("Transform::lookAt()", "[transform][lookAt]") {
  SECTION("looking at the origin") {
    const vec3 eye = vec3(1.0, 0.0, 0.0);
    const vec3 up = vec3(0.0, 1.0, 0.0);
    const vec3 center = vec3(0.0, 0.0, 0.0);
    const mat4 result = Transform::lookAt(eye, center, up);
    const mat4 expected = glm::lookAt(eye, center, up);
    REQUIRE(*(glm::value_ptr(result)) == *(glm::value_ptr(expected)));
  }
}

TEST_CASE("Transform::perspective()", "[transform][perspective]") {
  SECTION("with arbitrary inputs") {
    const float fovy = 54.78;
    const float aspect = 9.23;
    const float zNear = 2.31;
    const float zFar = 345.89;
    const mat4 result = Transform::perspective(fovy, aspect, zNear, zFar);
    const mat4 expected = glm::perspective(fovy, aspect, zNear, zFar);
    REQUIRE(*(glm::value_ptr(result)) == *(glm::value_ptr(expected)));
  }
}

TEST_CASE("Transform::scale()", "[transform][scale]") {
  SECTION("by a positive factor only in the x-direction") {
    const vec4 original = vec4(2.8, -1.5, 5.3, 1.0);
    const vec4 result = Transform::scale(2.0, 1.0, 1.0) * original;

    SECTION("x-component") {
      REQUIRE(result.x == Approx(5.6));
    }
    SECTION("y-component") {
      REQUIRE(result.y == Approx(-1.5));
    }
    SECTION("z-component") {
      REQUIRE(result.z == Approx(5.3));
    }
    SECTION("w-component") {
      REQUIRE(result.w == Approx(1.0));
    }
  }

  SECTION("by a uniform positive factor") {
    const vec4 original = vec4(2.8, -1.5, 5.3, 1.0);
    const vec4 result = Transform::scale(3.0, 3.0, 3.0) * original ;

    SECTION("x-component") {
      REQUIRE(result.x == Approx(8.4));
    }
    SECTION("y-component") {
      REQUIRE(result.y == Approx(-4.5));
    }
    SECTION("z-component") {
      REQUIRE(result.z == Approx(15.9));
    }
    SECTION("w-component") {
      REQUIRE(result.w == Approx(1.0));
    }
  }

  SECTION("mirroring the y-component") {
    const vec4 original = vec4(2.8, -1.5, 5.3, 1.0);
    const vec4 result = Transform::scale(1.0, -3.0, 1.0) * original ;

    SECTION("x-component") {
      REQUIRE(result.x == Approx(2.8));
    }
    SECTION("y-component") {
      REQUIRE(result.y == Approx(4.5));
    }
    SECTION("z-component") {
      REQUIRE(result.z == Approx(5.3));
    }
    SECTION("w-component") {
      REQUIRE(result.w == Approx(1.0));
    }
  }

  SECTION("decimating the z-component") {
    const vec4 original = vec4(2.8, -1.5, 5.3, 1.0);
    const vec4 result = Transform::scale(1.0, 1.0, 0.0) * original ;

    SECTION("x-component") {
      REQUIRE(result.x == Approx(2.8));
    }
    SECTION("y-component") {
      REQUIRE(result.y == Approx(-1.5));
    }
    SECTION("z-component") {
      REQUIRE(result.z == Approx(0.0));
    }
    SECTION("w-component") {
      REQUIRE(result.w == Approx(1.0));
    }
  }
}

TEST_CASE("Transform::translate()", "[transform][translate]") {
  SECTION("in the x direction") {
    const vec4 position = vec4(2.1, 3.5, -4.9, 1.0);
    vec4 result = Transform::translate(5.2, 0.0, 0.0) * position;
    result = result/(result.w);

    SECTION("x-component") {
      REQUIRE(result.x == Approx(7.3));
    }
    SECTION("y-component") {
      REQUIRE(result.y == Approx(3.5));
    }
    SECTION("z-component") {
      REQUIRE(result.z == Approx(-4.9));
    }
    SECTION("w-component") {
      REQUIRE(result.w == Approx(1.0));
    }
  }

  SECTION("diagonally") {
    const vec4 position = vec4(2.1, 3.5, -4.9, 1.0);
    vec4 result = Transform::translate(0.8, -1.9, -9.8) * position;
    result = result/(result.w);

    SECTION("x-component") {
      REQUIRE(result.x == Approx(2.9));
    }
    SECTION("y-component") {
      REQUIRE(result.y == Approx(1.6));
    }
    SECTION("z-component") {
      REQUIRE(result.z == Approx(-14.7));
    }
    SECTION("w-component") {
      REQUIRE(result.w == Approx(1.0));
    }
  }
}
