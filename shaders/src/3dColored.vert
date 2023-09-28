#version 450

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 out_color;

layout(binding = 0) uniform Ubo {
    mat4 model;
    mat4 view;
    mat4 proj;
};

void main()
{
    gl_Position =  proj * view * model * vec4(pos, 1.0f);
    out_color = color;
}