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

vec4 pointLight(vec3 light, float ambient, float specularIntensity, float specularPower) {
    vec4 lightColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    vec3 lightDirection = normalize(light - vPosition);
    float diffuse = max(dot(vNormal, lightDirection), 0.0f);

    vec3 viewDirection = normalize(-uCamPos - vPosition);
    vec3 reflectionDirection = reflect(-lightDirection, vNormal);
    float specularAmount = pow(max(dot(viewDirection, reflectionDirection), 0.0f), specularPower);
    float specular = specularAmount * specularIntensity;
    vec4 res = lightColor * (ambient + diffuse) + texture(uTex1, vTexCoord).r * specular;
    res.a = 1.0f;
    return res;
}

void main() {
    color = vec4(vColor, 1.0) * pointLight(vec3(80.0, 0.5, 10.0), 0.3, 1.5, 8.0);
}