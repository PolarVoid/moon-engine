#version 300 es
precision highp float;

uniform sampler2D uTex0;

in vec2 vTexCoord;
in vec3 vColor;

out vec4 color;


void main() {
    color = texture(uTex0, vTexCoord);
}