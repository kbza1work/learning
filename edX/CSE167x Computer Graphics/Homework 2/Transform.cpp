// Transform.cpp: implementation of the Transform class.

// Note: when you construct a matrix using mat4() or mat3(), it will be COLUMN-MAJOR
// Keep this in mind in readfile.cpp and display.cpp
// See FAQ for more details or if you're having problems.

#include "Transform.h"
#include <glm/gtc/type_ptr.hpp>

mat4 Transform::rotate(const float degrees, const vec3& axis)
{
  const vec3 nAxis = glm::normalize(axis);
  const float radians = glm::radians(degrees);
  const mat4 I(1.0);

  mat4 term1 = cos(radians) * I;
  term1[3][3] = 1.0;

  const mat4 term2 = (1 - cos(radians)) * mat4(
    nAxis.x * nAxis.x, nAxis.x * nAxis.y, nAxis.x * nAxis.z, 0.0,
    nAxis.x * nAxis.y, nAxis.y * nAxis.y, nAxis.y * nAxis.z, 0.0,
    nAxis.x * nAxis.z, nAxis.y * nAxis.z, nAxis.z * nAxis.z, 0.0,
    0.0,               0.0,               0.0,               0.0
  );

  const mat4 term3 = sin(radians) * mat4(
    0.0,            nAxis.z,        -1.0 * nAxis.y, 0.0,
    -1.0 * nAxis.z, 0.0,            nAxis.x,        0.0,
    nAxis.y,        -1.0 * nAxis.x, 0.0,            0.0,
    0.0,            0.0,            0.0,            0.0
  );

  return term1 + term2 + term3;
}

void Transform::left(float degrees, vec3& eye, vec3& up)
{
  const vec3 rotationAxis = glm::normalize(up);
  eye = mat3(Transform::rotate(degrees, rotationAxis)) * eye;
}

void Transform::up(float degrees, vec3& eye, vec3& up)
{
  const vec3 rotationAxis = glm::normalize(glm::cross(eye, up));
  eye = mat3(Transform::rotate(degrees, rotationAxis)) * eye;
  up = mat3(Transform::rotate(degrees, rotationAxis)) * up;
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
  // perspective matrix formula taken from the GLM documentation at
  // https://www.opengl.org/sdk/docs/man2/xhtml/gluPerspective.xml
  fovy = glm::radians(fovy);
  const float f = 1.0/glm::tan(fovy/2.0);
  return mat4(
    f/aspect, 0.0f,                            0.0f,  0.0f,
        0.0f,    f,                            0.0f,  0.0f,
        0.0f, 0.0f,   (zFar + zNear)/(zNear - zFar), -1.0f,
        0.0f, 0.0f, 2 * zFar * zNear/(zNear - zFar),  0.0f
  );
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

