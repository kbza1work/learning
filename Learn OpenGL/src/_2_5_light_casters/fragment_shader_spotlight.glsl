#version 330 core

struct Material {
	vec3 ambientColor;
	sampler2D diffuseColor;
	sampler2D specularColor;
	bool emissionPresent;
	sampler2D emissionColor;
	float shininess;
};

struct Light {
	vec3 ambientColor;
	vec3 diffuseColor;
	vec3 specularColor;
	vec3 directionView;
	float cutOffInner;	// the cosine of the splotlight's inner angle
	float cutOffOuter;	// the cosine of the splotlight's outer angle
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
	vec3 directionToLightInViewSpace = normalize(vertexPositionView - lightPositionView);
	vec3 baseColor = vec3(texture(material.diffuseColor, texCoords));

	vec3 finalColor;
	float theta = dot(directionToLightInViewSpace, normalize(light.directionView));
	float epsilon = light.cutOffInner - light.cutOffOuter;
	float intensity = clamp((theta - light.cutOffOuter)/epsilon, 0.0, 1.0);

	// ambient
	vec3 ambientColor = light.ambientColor * baseColor;

	// diffuse
	float diffuseFraction = max(dot(normalizedNormal, -directionToLightInViewSpace), 0.0);
	vec3 diffuseColor = intensity * light.diffuseColor * diffuseFraction * baseColor;

	// specular
	vec3 viewDirection = normalize(vertexPositionView);
	vec3 reflectionDirection = reflect(directionToLightInViewSpace, normal);
	float specularFraction = pow(max(dot(-viewDirection, reflectionDirection), 0.0), material.shininess);
	vec3 specularColor = intensity * light.specularColor * specularFraction * vec3(texture(material.specularColor, texCoords));

	// emission
	// vec3 emissionColor = vec3(texture(material.emissionColor, texCoords));
	vec3 emissionColor = vec3(0.0);
	if(material.emissionPresent) {
		emissionColor = vec3(texture(material.emissionColor, texCoords));
	}

	// DEBUG
	finalColor = baseColor;
	// finalColor = ambientColor + diffuseColor + specularColor + emissionColor;
	fragColor = vec4(finalColor, 1.0);
}
