# version 130

/* This is the fragment shader for reading in a scene description, including
   lighting.  Uniform lights are specified from the main program, and used in
   the shader.  As well as the material parameters of the object.  */

in vec4 color;
in vec3 normal;
in vec4 vertex;

const int numLights = 10;
uniform bool enablelighting;
uniform vec4 lightposn[numLights];  // positions of lights
uniform vec4 lightColor[numLights]; // colors of lights
uniform int numused;                // number of lights used

uniform mat4 modelViewMatrix;
uniform mat4 normalMatrix;

// Now, set the material parameters.  These could be varying and/or bound to
// a buffer.  But for now, I'll just make them uniform.
// I use ambient, diffuse, specular, shininess as in OpenGL.
// But, the ambient is just additive and doesn't multiply the lights.

uniform vec4 ambient;
uniform vec4 diffuse;
uniform vec4 specular;
uniform vec4 emission;
uniform float shininess;

// returns the color of the light produced on a surface with given properties
// from a single light
vec4 computeLight(
  const in vec3 lightDirection,
  const in vec4 lightColor,
  const in vec3 normal,
  const in vec3 halfVector,
  const in vec4 diffuse,
  const in vec4 specular,
  const in float shininess
) {
  float nDotL = max(dot(normal, lightDirection), 0.0);
  vec4 lambert = diffuse * nDotL;

  float nDotH = max(dot(normal, halfVector), 0.0);
  vec4 phong = specular * pow(nDotH, shininess);

  return lightColor * (lambert + phong);
}

void main (void)
{
    if(enablelighting) {
      vec4 finalColor = ambient;

      for(int thisLightIndex = 0; thisLightIndex < numused; thisLightIndex++) {
        vec4 worldLightPositionH = lightposn[thisLightIndex];

        bool isDirectionalLight = (abs(worldLightPositionH.w) < 0.00000001);

        const vec3 worldEyePositionIH = vec3(0, 0, 0);
        vec4 worldVertexH = modelViewMatrix * vertex;
        vec3 worldVertexIH = worldVertexH.xyz / worldVertexH.w;
        vec3 eyeDirectionIH = normalize(worldEyePositionIH - worldVertexIH);
        vec3 worldNormalIH = normalize((normalMatrix * vec4(normal, 0.0)).xyz);

        vec3 lightDirectionIH;
        if(isDirectionalLight) {
          lightDirectionIH = worldLightPositionH.xyz;
        } else {
          vec3 worldLightPositionIH =
            (worldLightPositionH/worldLightPositionH.w).xyz;
          lightDirectionIH = normalize(worldLightPositionIH - worldVertexIH);
        }

        vec3 halfVector = normalize(lightDirectionIH + eyeDirectionIH);

        finalColor += computeLight(
          lightDirectionIH,
          lightColor[thisLightIndex],
          worldNormalIH,
          halfVector,
          diffuse,
          specular,
          shininess
        );
      }

      gl_FragColor = finalColor;
    } else {
        gl_FragColor = color;
    }
}
