#version 450

layout(location = 0) in vec2 pos;
layout(location = 0) out vec2 out_pos;

layout( push_constant ) uniform Data {
    mat4 transform;
};

layout( set = 0, binding = 0) uniform CartesianToNorm {
    mat4 projection;
};

void main()
{
    out_pos = pos;
    gl_Position = projection * transform * vec4(pos, 0.0, 1.0);
}