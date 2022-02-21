#version 300 es

layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;

uniform float uTime;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProj;
uniform vec4 uColor;

out vec2 vTexCoord;
out vec3 vPosition;

void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
}