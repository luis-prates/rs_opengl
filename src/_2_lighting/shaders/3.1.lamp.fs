#version 330 core

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform Light light;
out vec4 FragColor;

void main()
{
    FragColor = vec4(light.diffuse, 1.0); // set all 4 vector values to 1.0
}
