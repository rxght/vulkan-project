#version 450

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 out_uv;

layout( push_constant ) uniform Ubo {
    mat4 mvp;
};

void main()
{
    gl_Position =  mvp * vec4(pos, 0.0f, 1.0f);
    out_uv = uv;
}