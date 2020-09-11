#version 430 core

out vec4 color;
in vec4 gl_FragCoord;

void main()
{
    //TASK 2d
    //Original shader
    //color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    //Modified shader
    //color = vec4(0.0f, 0.0f, 1.0f, 1.0f);

    //TASK 3a
    if (int(mod(gl_FragCoord[0]/10, 2)) == 0 && int(mod(gl_FragCoord[1]/10, 2)) == 1 ||
        int(mod(gl_FragCoord[0]/10, 2)) == 1 && int(mod(gl_FragCoord[1]/10, 2)) == 0 ) {
        color = vec4(1.0f, 0.0f, 1.0f, 1.0f);
    } else {
        color = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
}