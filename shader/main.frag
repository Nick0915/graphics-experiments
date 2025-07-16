#version 460 core

in vec3 vert_color;
out vec3 frag_color;

void main() {
    frag_color = vert_color;
    // frag_color = vec3(1.f, 1.f, 1.f);
}