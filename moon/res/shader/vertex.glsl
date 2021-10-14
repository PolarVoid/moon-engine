#version 300 es

in vec3 aPosition;

void main() {
    gl_Position = vec4(aPosition, 1.0);
    gl_Position.x += sin(float(gl_InstanceID)) * 0.1;
    gl_Position.y += cos(float(gl_InstanceID)) * 0.1;
}