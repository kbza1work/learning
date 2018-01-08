#version 330 core

uniform vec3 uOffset;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 vertexColor;

void main() {
    vec3 position = aPos + uOffset;
    gl_Position = vec4(position.x, position.y, position.z, 1.0);
	vertexColor = aColor;
}
