#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;
in layout(location=2) vec3 normals;

//Task 4
layout(location=3) uniform mat4 transformation;
layout(location=4) uniform mat4 normal_transformation;

out vec4 vertexColor;
out vec3 vertexNormals;

void main()
{
    vertexColor = color;
    vertexNormals = normalize(mat3(normal_transformation) * normals);
    gl_Position = transformation * vec4(position, 1.0);

}