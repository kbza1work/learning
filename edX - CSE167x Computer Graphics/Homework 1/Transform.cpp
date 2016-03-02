// Transform.cpp: implementation of the Transform class.

#include "Transform.h"

//Please implement the following functions:

// Helper rotation function.  
mat3 Transform::rotate(const float degrees, const vec3& axis) {
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

// Transforms the camera left around the "crystal ball" interface
void Transform::left(float degrees, vec3& eye, vec3& up) {
  const vec3 rotationAxis = glm::normalize(up);
  eye = Transform::rotate(degrees, rotationAxis) * eye;
}

// Transforms the camera up around the "crystal ball" interface
void Transform::up(float degrees, vec3& eye, vec3& up) {
  const vec3 rotationAxis = glm::normalize(glm::cross(eye, up));
  eye = Transform::rotate(degrees, rotationAxis) * eye;
  up = glm::normalize(Transform::rotate(degrees, rotationAxis) * up);
}

mat4 Transform::lookAt(vec3 eye, vec3 up) {
  const vec3 center = vec3(0.0, 0.0, 0.0);
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

Transform::Transform()
{

}

Transform::~Transform()
{

}
