#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 tex_coords;

layout(location = 0) out vec2 frag_tex_coords;

// MVP uniform
layout(set = 0, binding = 0) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
} mvp;

void main() {
    mat4 upside_down = mat4(1, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1);

    frag_tex_coords = tex_coords;
    gl_Position = mvp.proj * upside_down * mvp.view * mvp.model * vec4(position, 1.0);
}
