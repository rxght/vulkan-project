#version 450

layout(location = 0) in vec3 fragCol;
layout(location = 0) out vec4 outColor;

layout(binding = 0) uniform ubo {
    float brightness;
};

void main() {
    outColor = vec4(brightness * fragCol, 1.0);
}