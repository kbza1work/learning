#version 330 core

uniform mat4 modelMatrix;
uniform mat3 normalMatrixView;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

out vec3 normal;
out vec3 vertexPositionView;

void main() {
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(aPos, 1.0);
	normal = normalMatrixView * aNormal;
	vertexPositionView = (viewMatrix * modelMatrix * vec4(aPos, 1.0)).xyz;
}