#version 450

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 out_uv;

layout(binding = 0) uniform Ubo {
    mat4 model;
    mat4 view;
    mat4 proj;
};

void main()
{
    gl_Position =  proj * view * model * vec4(pos, 0.0f, 1.0f);
    out_uv = uv;
}