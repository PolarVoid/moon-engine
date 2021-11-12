#version 300 es
precision highp float;

uniform sampler2D uTex0;
uniform sampler2D uTex1;
uniform vec3 uCamPos;

in vec2 vTexCoord;
in vec3 vColor;
in vec3 vPosition;
in vec3 vNormal;

out vec4 color;

void main() {
    float ambient = 0.2f;
    vec3 light = vec3(0.0f, -0.5f, 0.0f);
    vec4 lightColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    vec3 lightDirection = normalize(light - vPosition);
    float diffuse = max(dot(vNormal, lightDirection), 0.0f);

    float specularIntensity = 1.0f;
    vec3 viewDirection = normalize(-uCamPos - vPosition);
    vec3 reflectionDirection = reflect(-lightDirection, vNormal);
    float specularAmount = pow(max(dot(viewDirection, reflectionDirection), 0.0f), 16.0f);
    float specular = specularAmount * specularIntensity;

    color = texture(uTex0, vTexCoord) * lightColor * (ambient + diffuse) + texture(uTex1, vTexCoord).r * specular;
    color.a = 1.0f;
}