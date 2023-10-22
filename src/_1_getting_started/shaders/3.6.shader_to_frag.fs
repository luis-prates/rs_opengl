#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec3 vectorPos;

void main()
{
    FragColor = vec4(vectorPos, 1.0f);
}
