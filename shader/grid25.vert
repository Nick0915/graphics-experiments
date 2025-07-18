#version 460 core

layout (location = 0) in vec3 position;

uniform mat4 u_proj_view;
uniform mat4 u_model[25 + 25 + 2];

out vec3 vert_color;

void main() {
    gl_Position = u_proj_view * u_model[gl_InstanceID] * vec4(position, 1.f);
    vert_color = vec3(1.f, 1.f, 1.f);
}


