#version 450

layout(location = 0) in vec3 frag_color;
layout(location = 1) in vec2 frag_tex_coords;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 1) uniform sampler2D tex_sampler;

void main() {
    vec4 tex = texture(tex_sampler, frag_tex_coords);
    out_color = tex;
    //out_color = tex_color * vec4(frag_color, 1.0); // This would tint the texture with vertex color, not needed.
}
