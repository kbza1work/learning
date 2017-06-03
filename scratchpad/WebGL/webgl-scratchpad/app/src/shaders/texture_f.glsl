#version 300 es
precision highp float;

uniform sampler2D uSampler;
uniform float uAlpha;

in vec2 textureCoord;

out vec4 fragColor;

void main(void) {
	vec4 textureColor = texture(
		uSampler, vec2(textureCoord.s, textureCoord.t)
	);
	fragColor = vec4(textureColor.rgb, textureColor.a * uAlpha);
}
