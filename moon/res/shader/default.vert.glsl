#version 300 es

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) in vec3 aNormal;

uniform float uTime;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProj;

out vec2 vTexCoord;
out vec3 vColor;
out vec3 vNormal;
out vec3 vPosition;

mat4 rotateY(float deg) {
    return mat4(cos(deg), 0, sin(deg), 0, 0, 1, 0, 0, -sin(deg), 0, cos(deg), 0, 0, 0, 0, 1);
}

void main() {
    gl_Position = uProj * uView * uModel * vec4(aPosition, 1.0);
    vPosition = vec3(uModel * vec4(aPosition, 1.0f));
    vColor = aColor;
    vTexCoord = aTexCoord;
    vTexCoord.y = 1.0 - vTexCoord.y;
    vNormal = normalize(aNormal);
}