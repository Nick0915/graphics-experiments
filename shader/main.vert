#version 460 core

layout (location = 0) in vec3 position;

uniform mat4 u_MVP;

out vec3 vert_color;

void main() {
    gl_Position = u_MVP * vec4(position, 1.f);
    vert_color = vec3(position);
}

