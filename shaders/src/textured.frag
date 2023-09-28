#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 out_color;

void main()
{
    out_color = vec4(uv, 0.0f, 1.0f);
}