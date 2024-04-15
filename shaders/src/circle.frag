#version 450

layout(location = 0) in vec2 realtive_pos;
layout(location = 0) out vec4 outColor;

void main()
{
    vec4 circle_color = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 background_color = vec4(1.0, 0.0, 0.0, 0.0);
    float blend_factor = step(1, length(realtive_pos));
    outColor = (1.0 - blend_factor) * circle_color + blend_factor * background_color;
}