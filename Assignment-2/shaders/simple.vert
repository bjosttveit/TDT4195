#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;

//Task 3 value
//uniform float value;

//Task 4
uniform mat4 transformation;

//==============TASK 1aii==============
out vec4 vertexColor;

void main()
{
    vertexColor = color;
    //gl_Position = vec4(position, 1.0);

    //==============TASK 3==============
    /*mat4 transformation = mat4(
        1.0, 0.0, 0.0, 0.0, 
        0.0, 1.0, 0.0, value, 
        0.0, 0.0, 1.0, 0.0, 
        0.0, 0.0, 0.0, 1.0
        );*/

    //Transposing the matrix so that it can be written in the same way as the formulas
    //gl_Position = transpose(transformation) * vec4(position, 1.0);

    //TASK 4
    gl_Position = transformation * vec4(position, 1.0);

}