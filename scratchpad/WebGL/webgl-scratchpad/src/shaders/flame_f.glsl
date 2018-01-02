#version 300 es

precision mediump float;

uniform vec4 uColor;
uniform float uTimeFrag;
uniform sampler2D fireAtlas;

in float lifetime;
in vec2 textureCoords;

out vec4 fragColor;

void main (void) {
	float time = mod(uTimeFrag, lifetime);
	float percentOfLife = time / lifetime;
	percentOfLife = clamp(percentOfLife, 0.0, 1.0);

	float offset = floor(16.0 * percentOfLife);
	float offsetX = floor(mod(offset, 4.0)) / 4.0;
	float offsetY = 0.75 - floor(offset / 4.0) / 4.0;

	vec4 texColor = texture(
		fireAtlas, 
		vec2(
			(textureCoords.x / 4.0) + offsetX,
			(textureCoords.y / 4.0) + offsetY
		)
	);
	fragColor = uColor * texColor;
	fragColor.a *= lifetime;
}
