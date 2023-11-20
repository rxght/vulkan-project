#version 450

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 out_uv;

layout( push_constant ) uniform Pc {
    mat4 model;
};

layout( set = 0, binding = 0 ) uniform GlobalUbo {
    mat4 view_projection;
};

void main()
{
    gl_Position =  view_projection * model * vec4(pos, 0.0f, 1.0f);
    out_uv = uv;
}