#version 300 es

precision lowp float;

in vec4 vertexColor;

out vec4 fragColor;

void main(void) {
	fragColor = vertexColor;
}
