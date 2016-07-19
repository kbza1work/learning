#version 110

uniform float fade_factor;

varying vec2 pos2;

void main()
{
	// swirling colors
	//
	//gl_FragColor = vec4(
	//	abs(pos2.x * fade_factor),
	//	0.0,
	//	0.0,
	//	1.0
	//);
	//gl_FragColor = vec4(
	//	sin(7.0 * fade_factor + pos2.x),
	//	sin(2.0 * fade_factor + pos2.y),
	//	fade_factor - pos2.x - pos2.y,
	//	1.0
	//);


	// spinner
	//
	float r = sqrt(pos2.x * pos2.x + pos2.y * pos2.y);
	float t = 0.0;
	t = atan(pos2.y, pos2.x);

	gl_FragColor = vec4(
		abs(cos(t * pos2.x * fade_factor)),
		abs(cos(t * fade_factor)),
		abs(cos(t * pos2.y * fade_factor)),
		1.0
	);
}
