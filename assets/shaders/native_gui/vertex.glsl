#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec4 frag_color;
layout(location = 1) out vec2 out_uv;

void main() {
    frag_color = color;
    out_uv = uv;

    gl_Position = vec4(position, 1.0);
}
