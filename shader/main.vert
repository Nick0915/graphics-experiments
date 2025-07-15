#version 460 core

layout (location = 0) in vec2 pos;

// uniform mat4 u_MVP;

void main() {
    // gl_Position = u_MVP * vec4(pos, 1.f);
    gl_Position = vec4(pos.x, pos.y, 0.f, 1.f);
}

