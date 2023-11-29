#version 450

layout(location = 0) out vec4 out_color;

layout( push_constant ) uniform Color {
    mat4 padding;
    vec3 color;
};

void main()
{
    out_color = vec4(color, 1.0);
}