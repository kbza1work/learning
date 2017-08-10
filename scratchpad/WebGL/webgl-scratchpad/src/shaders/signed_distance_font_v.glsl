#version 300 es

in vec3 aVertexPosition;
in vec2 aTextureCoordinate;

out vec2 textureCoordinate;

uniform mat4 modelViewMatrix;
uniform mat4 projectionMatrix;

void main(void) {
	// vec2 glyphPosition = vec2(
	// 	(codePoint % glyphsPerColumn) * glyphPixelWidth,
	// 	(codePoint / glyphsPerColumn) * glyphPixelHeight
	// );
	gl_Position = projectionMatrix * modelViewMatrix * vec4(aVertexPosition, 1.0);
	textureCoordinate = aTextureCoordinate;
}
