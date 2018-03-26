#version 330 core

struct Material {
	vec3 ambientColor;
	sampler2D diffuseColor;
	sampler2D specularColor;
	sampler2D emissionColor;
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
in vec2 texCoords;

out vec4 fragColor;

void main() {
	vec3 normalizedNormal = normalize(normal);
	// this is the incident vector of the light on the surface
	vec3 lightDirectionView = normalize(vertexPositionView - lightPositionView);
	vec3 baseColor = vec3(texture(material.diffuseColor, texCoords));

	// ambient
	vec3 ambientColor = light.ambientColor * baseColor;

	// diffuse
	float diffuseFraction = max(dot(normalizedNormal, -lightDirectionView), 0.0);
	vec3 diffuseColor = light.diffuseColor * diffuseFraction * baseColor;

	// specular
	vec3 viewDirection = normalize(vertexPositionView);
	vec3 reflectionDirection = reflect(lightDirectionView, normal);
	float specularFraction = pow(max(dot(-viewDirection, reflectionDirection), 0.0), material.shininess);
	vec3 specularColor = light.specularColor * specularFraction * vec3(texture(material.specularColor, texCoords));

	// emission
	vec3 emissionColor = vec3(texture(material.emissionColor, texCoords));

	vec3 finalColor = ambientColor + diffuseColor + specularColor + emissionColor;
	fragColor = vec4(finalColor, 1.0);
}
