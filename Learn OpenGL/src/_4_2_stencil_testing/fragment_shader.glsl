#version 330 core

struct Material {
	vec3 ambientColor;
	sampler2D diffuseColor;
	sampler2D specularColor;
	bool emissionPresent;
	sampler2D emissionColor;
	float shininess;
};

struct PointLight {
	vec3 positionView;
	vec3 ambientColor;
	vec3 diffuseColor;
	vec3 specularColor;
	float constant;
	float linear;
	float quadratic;
};

struct DirectionalLight {
	vec3 positionView;
	vec3 ambientColor;
	vec3 diffuseColor;
	vec3 specularColor;
};

struct Spotlight {
	vec3 positionView;
	vec3 ambientColor;
	vec3 diffuseColor;
	vec3 specularColor;
	vec3 directionView;
	float cutOffInner;	// the cosine of the splotlight's inner angle
	float cutOffOuter;	// the cosine of the splotlight's outer angle
};

uniform Material material;
#define NUM_POINT_LIGHTS 4
uniform PointLight pointLights[NUM_POINT_LIGHTS];
uniform DirectionalLight directionalLight;
uniform Spotlight spotlight;

in vec3 vertexPositionView;
in vec3 normal;
in vec2 texCoords;

out vec4 fragColor;

vec3 pointLightColor(
	PointLight light,
	Material material,
	vec3 vertexPositionView,
	vec3 normalizedNormal
);
vec3 directionalLightColor(
	DirectionalLight light,
	Material material,
	vec3 vertexPositionView,
	vec3 normalizedNormal
);
vec3 spotlightColor(
	Spotlight light,
	Material material,
	vec3 vertexPositionView,
	vec3 normalizedNormal
);

void main() {
	vec3 normalizedNormal = normalize(normal);

	vec3 color = vec3(0.0);
	// DEBUG -- only one light
	// for(int i = 0; i < 1; i++) {
	for(int i = 0; i < NUM_POINT_LIGHTS; i++) {
		color += pointLightColor(pointLights[i], material, vertexPositionView, normalizedNormal);
	}
	color += directionalLightColor(directionalLight, material, vertexPositionView, normalizedNormal);
	color += spotlightColor(spotlight, material, vertexPositionView, normalizedNormal);

	// emission
	if(material.emissionPresent) {
		color += vec3(texture(material.emissionColor, texCoords));
	}

	fragColor = vec4(color, 1.0);
}

vec3 pointLightColor(
	PointLight light,
	Material material,
	vec3 vertexPositionView,
	vec3 normalizedNormal
) {
	// this is the incident vector of the light on the surface
	vec3 directionToLightInViewSpace = normalize(vertexPositionView - light.positionView);
	vec3 baseColor = vec3(texture(material.diffuseColor, texCoords));

	// attenuation
	float distance = length(light.positionView - vertexPositionView);
	float attenuation = 1.0 / (light.constant + (light.linear * distance) + (light.quadratic * distance * distance));

	// ambient
	vec3 ambientColor = attenuation * light.ambientColor * baseColor;

	// diffuse
	float diffuseFraction = max(dot(normalizedNormal, -directionToLightInViewSpace), 0.0);
	vec3 diffuseColor = attenuation * light.diffuseColor * diffuseFraction * baseColor;

	// specular
	vec3 viewDirection = normalize(vertexPositionView);
	vec3 reflectionDirection = reflect(directionToLightInViewSpace, normal);
	float specularFraction = pow(max(dot(-viewDirection, reflectionDirection), 0.0), material.shininess);
	vec3 specularColor = attenuation * light.specularColor * specularFraction * vec3(texture(material.specularColor, texCoords));

	return ambientColor + diffuseColor + specularColor;
}

vec3 directionalLightColor(
	DirectionalLight light,
	Material material,
	vec3 vertexPositionView,
	vec3 normalizedNormal
) {
	// this is the incident vector of the light on the surface
	vec3 directionToLightInViewSpace = light.positionView;
	vec3 baseColor = vec3(texture(material.diffuseColor, texCoords));

	// ambient
	vec3 ambientColor = light.ambientColor * baseColor;

	// diffuse
	float diffuseFraction = max(dot(normalizedNormal, -directionToLightInViewSpace), 0.0);
	vec3 diffuseColor = light.diffuseColor * diffuseFraction * baseColor;

	// specular
	vec3 viewDirection = normalize(vertexPositionView);
	vec3 reflectionDirection = reflect(directionToLightInViewSpace, normal);
	float specularFraction = pow(max(dot(-viewDirection, reflectionDirection), 0.0), material.shininess);
	vec3 specularColor = light.specularColor * specularFraction * vec3(texture(material.specularColor, texCoords));

	return ambientColor + diffuseColor + specularColor;
}

vec3 spotlightColor(
	Spotlight light,
	Material material,
	vec3 vertexPositionView,
	vec3 normalizedNormal
) {
	// this is the incident vector of the light on the surface
	vec3 directionToLightInViewSpace = normalize(vertexPositionView - light.positionView);
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

	return ambientColor + diffuseColor + specularColor;
}
