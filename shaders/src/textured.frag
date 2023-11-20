#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 out_color;

layout(set = 1, binding = 0) uniform sampler2D tex;

void main()
{
    out_color = texture(tex, uv);
}