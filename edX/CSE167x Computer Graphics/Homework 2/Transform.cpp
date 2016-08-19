// Transform.cpp: implementation of the Transform class.

// Note: when you construct a matrix using mat4() or mat3(), it will be COLUMN-MAJOR
// Keep this in mind in readfile.cpp and display.cpp
// See FAQ for more details or if you're having problems.

#include "Transform.h"
#include <glm/gtc/type_ptr.hpp>

// Helper rotation function.  Please implement this.
mat3 Transform::rotate(const float degrees, const vec3& axis)
{
  const float radians = glm::radians(degrees);
  const mat3 I(1.0);

  const mat3 term1 = cos(radians) * I;
  const mat3 term2 = (1 - cos(radians)) * mat3(
    axis.x * axis.x, axis.x * axis.y, axis.x * axis.z,
    axis.x * axis.y, axis.y * axis.y, axis.y * axis.z,
    axis.x * axis.z, axis.y * axis.z, axis.z * axis.z
  );
  const mat3 term3 = sin(radians) * mat3(
    0.0, axis.z, -1.0 * axis.y,
    -1.0 * axis.z, 0.0, axis.x,
    axis.y, -1.0 * axis.x, 0.0
  );

  return term1 + term2 + term3;
}

void Transform::left(float degrees, vec3& eye, vec3& up)
{
  const vec3 rotationAxis = glm::normalize(up);
  eye = Transform::rotate(degrees, rotationAxis) * eye;
}

void Transform::up(float degrees, vec3& eye, vec3& up)
{
  const vec3 rotationAxis = glm::normalize(glm::cross(eye, up));
  eye = Transform::rotate(degrees, rotationAxis) * eye;
  up = Transform::rotate(degrees, rotationAxis) * up;
}

mat4 Transform::lookAt(const vec3 &eye, const vec3 &center, const vec3 &up)
{
  const vec3 f = glm::normalize(vec3(
    center.x - eye.x, center.y - eye.y, center.z - eye.z
  ));
  const vec3 s = glm::cross(f, glm::normalize(up));
  const vec3 u = glm::cross(glm::normalize(s), f);

  const vec4 column0 = vec4(s.x, u.x, -1.0 * f.x, 0.0);
  const vec4 column1 = vec4(s.y, u.y, -1.0 * f.y, 0.0);
  const vec4 column2 = vec4(s.z, u.z, -1.0 * f.z, 0.0);
  const vec4 column3 = vec4(
    -s.x * eye.x - s.y * eye.y - s.z * eye.z,
    -u.x * eye.x - u.y * eye.y - u.z * eye.z,
    f.x * eye.x + f.y * eye.y + f.z * eye.z, 1.0
  );

  return mat4(column0, column1, column2, column3);
}

mat4 Transform::perspective(float fovy, float aspect, float zNear, float zFar)
{
  mat4 ret;
  // YOUR CODE FOR HW2 HERE
  // New, to implement the perspective transform as well.
  return ret;
}

mat4 Transform::scale(const float &sx, const float &sy, const float &sz)
{
    return mat4(
        sx,  0.0, 0.0, 0.0,
        0.0, sy,  0.0, 0.0,
        0.0, 0.0, sz,  0.0,
        0.0, 0.0, 0.0, 1.0
    );
}

mat4 Transform::translate(const float &tx, const float &ty, const float &tz)
{
    return mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        tx,  ty,  tz,  1.0
    );
}

// To normalize the up direction and construct a coordinate frame.
// As discussed in the lecture.  May be relevant to create a properly
// orthogonal and normalized up.
// This function is provided as a helper, in case you want to use it.
// Using this function (in readfile.cpp or display.cpp) is optional.

vec3 Transform::upvector(const vec3 &up, const vec3 & zvec)
{
  vec3 x = glm::cross(up,zvec);
  vec3 y = glm::cross(zvec,x);
  vec3 ret = glm::normalize(y);
  return ret;
}


Transform::Transform()
{

}

Transform::~Transform()
{

}

