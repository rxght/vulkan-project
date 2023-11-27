#version 450

layout(location = 0) in vec2 position;

layout( push_constant ) uniform Pc {
    vec2 scaling;
};

void main()
{
    gl_Position =  vec4(scaling * position, 0.0f, 1.0f);
}