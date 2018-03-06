#version 330 core

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 ambientColor;
uniform vec3 lightPositionView;

in vec3 vertexPositionView;
in vec3 normal;

out vec4 fragColor;

void main() {
	vec3 normalizedNormal = normalize(normal);
	// this is the incident vector of the light on the surface
	vec3 lightDirectionView = normalize(vertexPositionView - lightPositionView);

	float diffuseFraction = max(dot(normalizedNormal, -lightDirectionView), 0.0);
	vec3 diffuseColor = diffuseFraction * lightColor;

	float specularStrength = 0.5;
	float shininess = 32.0;
	vec3 viewDirection = normalize(vertexPositionView);
	vec3 reflectionDirection = reflect(lightDirectionView, normal);
	float specularFraction = pow(max(dot(-viewDirection, reflectionDirection), 0.0), shininess);
	vec3 specularColor = specularStrength * specularFraction * lightColor;

	vec3 finalColor = (ambientColor + diffuseColor + specularColor) * objectColor;
	fragColor = vec4(finalColor, 1.0);
}
