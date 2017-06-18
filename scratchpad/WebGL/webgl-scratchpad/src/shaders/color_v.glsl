#version 300 es

in vec3 aVertexPosition;
in vec4 aVertexColor;

out vec4 vertexColor;

uniform mat4 modelViewMatrix;
uniform mat4 perspectiveMatrix;

void main(void) {
	gl_Position = perspectiveMatrix * modelViewMatrix * vec4(aVertexPosition, 1.0);
	vertexColor = aVertexColor;
}
