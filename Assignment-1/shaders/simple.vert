#version 430 core

in vec3 position;

void main()
{
    //TASK 2d
    //Original shader
    //gl_Position = vec4(position, 1.0f);
    //Modified shader
    gl_Position = vec4(-position, 1.0f);
}