#version 300 es

precision mediump float;

uniform sampler2D uSampler;
uniform vec3 uColor;

in vec2 textureCoord;

out vec4 fragColor;

void main(void) {
	vec4 textureColor = texture(
		uSampler, vec2(textureCoord.s, textureCoord.t)
	);
	fragColor = textureColor * vec4(uColor, 1.0);
}
