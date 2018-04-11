#version 330 core

struct Light {
	vec3 ambientColor;
	vec3 diffuseColor;
	vec3 specularColor;
};

uniform Light light;

out vec4 fragColor;

void main() {
	fragColor = vec4(light.specularColor, 1.0);
}
