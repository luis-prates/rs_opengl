#version 330 core
out vec4 FragColor;

struct Material {
    float shininess;
};

struct Light {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform vec3 viewPos;
uniform Material material;
uniform Light light;
uniform sampler2D diffuse;
uniform sampler2D specular;
uniform sampler2D emission;
uniform float time;

void main()
{
    // ambient
    vec3 ambient = light.ambient * texture(diffuse, TexCoords).rgb;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * texture(diffuse, TexCoords).rgb;

	// emission
	vec3 emission = texture(emission, TexCoords + vec2(0.0, time)).rgb * floor(vec3(1.f) - texture(specular,TexCoords).rgb);

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * texture(specular, TexCoords).rgb;

    vec3 result = ambient + diffuse + specular + emission;
    FragColor = vec4(result, 1.0);
}