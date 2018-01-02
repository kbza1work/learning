#version 300 es

precision mediump float;

in vec2 textureCoordinate;

uniform sampler2D msdf;
uniform vec4 color;
uniform vec4 bgColor;
uniform float pxRange;

out vec4 fragColor;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

void main() {
    vec2 msdfUnit = pxRange/vec2(textureSize(msdf, 0));
    vec3 msdfSample = texture(msdf, textureCoordinate).rgb;
    float sigDist = median(msdfSample.r, msdfSample.g, msdfSample.b) - 0.5;
    sigDist *= dot(msdfUnit, 0.5/fwidth(textureCoordinate));
    float opacity = clamp(sigDist + 0.5, 0.0, 1.0);
    fragColor = mix(bgColor, color, opacity);

	// DEBUG - comment out everything else in main() and uncomment this section
	// to use the msdf itself as the texture
	//vec4 textureColor = texture(
	//	msdf, vec2(textureCoordinate.s, textureCoordinate.t)
	//);
	//fragColor = vec4(textureColor.rgb, 1.0);
}
