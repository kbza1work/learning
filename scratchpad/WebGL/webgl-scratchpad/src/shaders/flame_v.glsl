#version 300 es

in float aLifetime;
in vec2 aTextureCoords;
in vec2 aTriCorner;
in vec3 aCenterOffset;
in vec3 aVelocity;

out float lifetime;
out vec2 textureCoords;

uniform mat4 modelViewMatrix;
uniform mat4 perspectiveMatrix;
uniform float uTime;
uniform vec3 uFirePos;

void main (void) {
	float time = mod(uTime, aLifetime);

	vec4 position = vec4(uFirePos + aCenterOffset + (time * aVelocity), 1.0);

	lifetime = clamp(1.3 - (time / aLifetime), 0.0, 1.0);
	float size = (lifetime * lifetime) * 0.15;

	vec3 cameraRight = vec3(modelViewMatrix[0].x, modelViewMatrix[1].x, modelViewMatrix[2].x);
	vec3 cameraUp = vec3(modelViewMatrix[0].y, modelViewMatrix[1].y, modelViewMatrix[2].y);

	position.xyz +=
		(cameraRight * aTriCorner.x * size) + (cameraUp * aTriCorner.y * size);

	gl_Position = perspectiveMatrix * modelViewMatrix * position;

	textureCoords = aTextureCoords;
	lifetime = aLifetime;
}
