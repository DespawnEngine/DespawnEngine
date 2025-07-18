#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

layout(location = 0) out vec3 frag_color;

// MVP uniform
layout(set = 0, binding = 0) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
} mvp;

void main() {
    frag_color = color;
    gl_Position = mvp.proj * mvp.view * mvp.model * vec4(position, 1.0);
}
