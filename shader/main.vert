#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texcoord;
layout (location = 2) in vec3 normal;

uniform mat4 u_MVP;

out vec3 vert_color;

void main() {
    gl_Position = u_MVP * vec4(position, 1.f);
    vert_color = vec3(normal);
}

