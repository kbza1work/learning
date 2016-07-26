#include <cmath>
#include <string>
#include <iostream>
#include <sstream>
#include <glm/glm.hpp>

#include "Transform.h"

const float EPSILON = 0.1;

std::string format(const vec3& vector) {
	std::ostringstream text;
	text << "(" << vector.x << ", " << vector.y << ", " << vector.z << ")";
	return text.str();
}

std::string format(const vec4& vector) {
	std::ostringstream text;
	text << "(" << vector[0] << ", " << vector[1] << ", " << vector[2] << ", " << vector[3] << ")";
	return text.str();
}

std::string format(const mat4& matrix) {
	std::ostringstream text;
	text << "[" << format(matrix[0]) << "]\n"
		<< "[" << format(matrix[1]) << "]\n"
		<< "[" << format(matrix[2]) << "]\n"
		<< "[" << format(matrix[3]) << "]" << std::endl;
	return text.str();
}

bool eq(const float a, const float b, const float epsilon) {
	return abs(a - b) < epsilon;
}

bool eq(const vec3& a, const vec3& b, const float epsilon) {
	return (
		eq(a.x, b.x, epsilon) &&
		eq(a.y, b.y, epsilon) &&
		eq(a.z, b.z, epsilon)
	);
}

bool eq(const vec4& a, const vec4& b, const float epsilon) {
	return (
		eq(a[0], b[0], epsilon) &&
		eq(a[1], b[1], epsilon) &&
		eq(a[2], b[2], epsilon) &&
		eq(a[3], b[3], epsilon)
	);
}

bool eq(const mat4& a, const mat4& b, const float epsilon) {
	return (
		eq(a[0], b[0], epsilon) &&
		eq(a[1], b[1], epsilon) &&
		eq(a[2], b[2], epsilon) &&
		eq(a[3], b[3], epsilon)
	);
}

bool testAssert(const vec3& expected, const vec3& actual, const std::string& message) {
	if(eq(expected, actual, EPSILON)) {
		std::cout << "\t\033[0;32mPASSED\033[0m" << std::endl;
		return true;
	} else {
		std::cout << "\t\033[1;31mFAILED\033[0m (" << message << ")\n\texpected: ";
		std::cout << format(expected) << "; actual: " << format(actual) << std::endl;
		return false;
	}
}

bool testAssert(const vec4& expected, const vec4& actual, const std::string& message) {
	if(eq(expected, actual, EPSILON)) {
		std::cout << "\t\033[0;32mPASSED\033[0m" << std::endl;
		return true;
	} else {
		std::cout << "\t\033[1;31mFAILED\033[0m (" << message << ")\n\texpected: ";
		std::cout << format(expected) << "; actual: " << format(actual) << std::endl;
		return false;
	}
}

bool testAssert(const mat4& expected, const mat4& actual, const std::string& message) {
	if(eq(expected, actual, EPSILON)) {
		std::cout << "\t\033[0;32mPASSED\033[0m" << std::endl;
		return true;
	} else {
		std::cout << "\t\033[1;31mFAILED\033[0m (" << message << ")"
			<< "\nexpected:\n" << format(expected)
			<< "\nactual:\n" << format(actual)
			<< std::endl;
		return false;
	}
}

void leftByPositiveAngle() {
	std::cout << "leftByPositiveAngle:" << std::endl;
	const float angle = 90.0;
	vec3 eye = vec3(0.0, 0.0, 5.0);
	vec3 up = vec3(0.0, 1.0, 0.0);
	Transform::left(angle, eye, up);
	testAssert(vec3(5.0, 0.0, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(0.0, 1.0, 0.0), up, "up coordinates incorrect");
}

void leftByNegativeAngle() {
	std::cout << "leftByNegativeAngle:" << std::endl;
	const float angle = -90.0;
	vec3 eye = vec3(0.0, 0.0, 5.0);
	vec3 up = vec3(0.0, 1.0, 0.0);
	Transform::left(angle, eye, up);
	testAssert(vec3(-5.0, 0.0, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(0.0, 1.0, 0.0), up, "up coordinates incorrect");
}

void leftByCantedPositiveAngle() {
	std::cout << "leftByCantedPositiveAngle:" << std::endl;
	const float angle = 90.0;
	vec3 eye = vec3(0.0, 0.0, 5.0);
	vec3 up = vec3(0.5, 0.5, 0.0);
	Transform::left(angle, eye, up);
	testAssert(vec3(2.236067977, -2.236067977, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(0.5, 0.5, 0.0), up, "up coordinates incorrect");
}

void leftByRepeatedPositiveAngle() {
	std::cout << "leftByRepeatedPositiveAngle:" << std::endl;
	const float angle = 90.0;
	vec3 eye = vec3(0.0, 0.0, 5.0);
	vec3 up = vec3(0.5, 0.5, 0.0);
	for(int i = 0; i < 32; i++) {
		Transform::left(angle, eye, up);
	}
	testAssert(vec3(2.236067977, -2.236067977, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(0.5, 0.5, 0.0), up, "up coordinates incorrect");
}

void upByPositiveAngle() {
	std::cout << "upByPositiveAngle:" << std::endl;
	const float angle = 90.0;
	vec3 eye = vec3(0.0, 0.0, 5.0);
	vec3 up = vec3(0.0, 1.0, 0.0);
	Transform::up(angle, eye, up);
	testAssert(vec3(0.0, 5.0, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(0.0, 0.0, -1.0), up, "up coordinates incorrect");
}

void upByNegativeAngle() {
	std::cout << "upByNegativeAngle:" << std::endl;
	const float angle = -90.0;
	vec3 eye = vec3(0.0, 0.0, 5.0);
	vec3 up = vec3(0.0, 1.0, 0.0);
	Transform::up(angle, eye, up);
	testAssert(vec3(0.0, -5.0, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(0.0, 0.0, 1.0), up, "up coordinates incorrect");
}

void upByCantedPositiveAngle() {
	std::cout << "upByCantedPositiveAngle:" << std::endl;
	const float angle = 90.0;
	vec3 eye = vec3(2.236067977, 2.236067977, 0.0);
	vec3 up = vec3(-0.5, 0.5, 0.0);
	Transform::up(angle, eye, up);
	testAssert(vec3(-2.236067977, 2.236067977, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(-0.5, -0.5, 0.0), up, "up coordinates incorrect");
}

void upByRepeatedPositiveAngle() {
	std::cout << "upByRepeatedPositiveAngle:" << std::endl;
	const float angle = 90.0;
	vec3 eye = vec3(2.236067977, 2.236067977, 0.0);
	vec3 up = vec3(-0.5, 0.5, 0.0);
	for(int i = 0; i < 32; i++) {
		Transform::up(angle, eye, up);
	}
	testAssert(vec3(-2.236067977, 2.236067977, 0.0), eye, "eye coordinates incorrect");
	testAssert(vec3(-0.5, -0.5, 0.0), up, "up coordinates incorrect");
}

void lookAtOrigin() {
	std::cout << "lookAt:" << std::endl;
	const vec3 eye = vec3(1.0, 0.0, 0.0);
	const vec3 up = vec3(0.0, 1.0, 0.0);
	const vec3 center = vec3(0.0, 0.0, 0.0);
	const mat4 result = Transform::lookAt(eye, center, up);
	testAssert(glm::lookAt(eye,center,up), result, "lookAt matrix incorrect");
}

void translateInXDirection() {
	std::cout << "translateInXDirection:" << std::endl;
	const vec4 position = vec4(2.1, 3.5, -4.9, 1.0);
	const vec4 result = position * Transform::translate(5.2, 0.0, 0.0);
	const vec4 expected = vec4(7.3, 3.5, -4.9, 1.0);
	testAssert(expected, result, "translation matrix incorrect");
}

void translateDiagonal() {
	std::cout << "translateInXDirection:" << std::endl;
	const vec4 position = vec4(2.1, 3.5, -4.9, 1.0);
	const vec4 result = position * Transform::translate(0.8, -1.9, -9.8);
	const vec4 expected = vec4(2.9, 1.6, -14.7, 1.0);
	testAssert(expected, result, "translation matrix incorrect");
}

int main(int argc,char* argv[]) {
	leftByPositiveAngle();
	leftByNegativeAngle();
	leftByCantedPositiveAngle();
	leftByRepeatedPositiveAngle();
	upByPositiveAngle();
	upByNegativeAngle();
	upByCantedPositiveAngle();
	upByRepeatedPositiveAngle();
	lookAtOrigin();
	translateInXDirection();
	translateDiagonal();
}
