#version 450

layout(location = 0) in vec2 position;

layout( push_constant ) uniform PointData {
    vec2 point_position;
    float radius;
};

layout( set = 0, binding = 0) uniform CartesianToNorm {
    mat4 projection;
};

void main()
{
    gl_Position = projection * vec4((position * radius) + point_position, 5.0, 1.0);
}