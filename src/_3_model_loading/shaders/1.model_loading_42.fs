#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
in vec3 ourColor;

uniform sampler2D texture_diffuse1;
uniform int useTexturing;
uniform float mixValue;

void main()
{
	// if (useTexturing == 1)
    // 	FragColor = texture(texture_diffuse1, TexCoords);
	// else
		FragColor = mix(texture(texture_diffuse1, TexCoords), vec4(ourColor, 1.0f), mixValue);
}
