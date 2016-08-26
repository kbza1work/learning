# version 130

uniform mat4 modelViewProjectionMatrix;
out vec4 color ;
out vec3 mynormal ;
out vec4 myvertex ;

void main() {
    gl_Position = modelViewProjectionMatrix * gl_Vertex;
    color = gl_Color ;
    mynormal = gl_Normal ;
    myvertex = gl_Vertex ;
}
