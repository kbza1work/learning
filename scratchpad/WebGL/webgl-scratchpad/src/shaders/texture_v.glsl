#version 300 es

in vec3 aVertexPosition;
in vec2 aTextureCoord;

out vec2 textureCoord;

uniform mat4 modelViewMatrix;
uniform mat4 perspectiveMatrix;

void main(void) {
	gl_Position = perspectiveMatrix * modelViewMatrix * vec4(aVertexPosition, 1.0);
	textureCoord = aTextureCoord;
}
