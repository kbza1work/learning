# version 130

uniform mat4 modelViewMatrix;
uniform mat4 normalMatrix;
uniform mat4 modelViewProjectionMatrix;
out vec4 color;
out vec3 normal;
out vec4 vertex;

void main() {
  gl_Position = modelViewProjectionMatrix * gl_Vertex;
  color = gl_Color ;
  normal = gl_Normal ;
  vertex = gl_Vertex ;
}
