#version 330 core

struct Material {
    vec3 ambientColor;
    vec3 diffuseColor;
    vec3 specularColor;
    float shininess;
};

struct Light {
	vec3 ambientColor;
	vec3 diffuseColor;
	vec3 specularColor;
};

uniform vec3 lightPositionView;
uniform Material material;
uniform Light light;

in vec3 vertexPositionView;
in vec3 normal;

out vec4 fragColor;

void main() {
	vec3 normalizedNormal = normalize(normal);
	// this is the incident vector of the light on the surface
	vec3 lightDirectionView = normalize(vertexPositionView - lightPositionView);

	// ambient
	vec3 ambientColor = light.ambientColor * material.ambientColor;

	// diffuse
	float diffuseFraction = max(dot(normalizedNormal, -lightDirectionView), 0.0);
	vec3 diffuseColor = light.diffuseColor * (diffuseFraction * material.diffuseColor);

	// specular
	vec3 viewDirection = normalize(vertexPositionView);
	vec3 reflectionDirection = reflect(lightDirectionView, normal);
	float specularFraction = pow(max(dot(-viewDirection, reflectionDirection), 0.0), material.shininess);
	vec3 specularColor = light.specularColor * (specularFraction * material.specularColor);

	vec3 finalColor = ambientColor + diffuseColor + specularColor;
	fragColor = vec4(finalColor, 1.0);
}
