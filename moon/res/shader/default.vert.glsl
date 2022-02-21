#version 300 es

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec2 aTexCoord;

uniform float uTime;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProj;
uniform vec4 uColor;

out vec2 vTexCoord;
out vec3 vPosition;

mat4 rotateY(float deg) {
    return mat4(cos(deg), 0, sin(deg), 0, 0, 1, 0, 0, -sin(deg), 0, cos(deg), 0, 0, 0, 0, 1);
}

void main() {
    if (gl_VertexID == 0) {
        gl_Position = vec4(-0.5, -0.5, 0.0, 1.0);
    } else if (gl_VertexID == 1) {
        gl_Position = vec4(-0.5, 0.5, 0.0, 1.0);
    } else if (gl_VertexID == 2) {
        gl_Position = vec4(0.5, 0.5, 0.0, 1.0);
    } else if (gl_VertexID == 3) {
        gl_Position = vec4(0.5, -0.5, 0.0, 1.0);
    }
}