#version 330 core
out vec4 FragColor;

struct Material {
    float shininess;
};

struct Light {
    vec3 position;
	vec3 direction;
	float cutOff;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

	float constant;
	float linear;
	float quadratic;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform vec3 viewPos;
uniform Material material;
uniform Light light;
uniform sampler2D diffuse;
uniform sampler2D specular;

void main()
{

	vec3 lightDir = normalize(light.position - FragPos);

	float theta = dot(lightDir, normalize(-light.direction));

	if (theta > light.cutOff)
	{
		// ambient
		vec3 ambient = light.ambient * texture(diffuse, TexCoords).rgb;

		// diffuse
		vec3 norm = normalize(Normal);
		float diff = max(dot(norm, lightDir), 0.0);
		vec3 diffuse = light.diffuse * diff * texture(diffuse, TexCoords).rgb;

		// specular
		vec3 viewDir = normalize(viewPos - FragPos);
		vec3 reflectDir = reflect(-lightDir, norm);
		float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
		vec3 specular = light.specular * spec * texture(specular, TexCoords).rgb;

		// attenuation
		float distance = length(light.position - FragPos);
		float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

		ambient  *= attenuation;
		diffuse   *= attenuation;
		specular *= attenuation;

		// do lighting calculations
		vec3 result = ambient + diffuse + specular;
		FragColor = vec4(result, 1.0);
	}
	else
		FragColor = vec4(light.ambient * vec3(texture(diffuse, TexCoords)), 1.0);

}