#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 tex_coords;

layout(location = 0) out vec3 frag_color;
layout(location = 1) out vec2 frag_tex_coords;

// MVP uniform
layout(set = 0, binding = 0) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
} mvp;

void main() {
    frag_color = color;
    frag_tex_coords = tex_coords;
    gl_Position = mvp.proj * mvp.view * mvp.model * vec4(position, 1.0);
}
