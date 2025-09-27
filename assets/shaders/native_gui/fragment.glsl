#version 450

layout(location = 0) in vec4 frag_color;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform sampler2D test_sampler;


void main() {
  out_color = vec4(texture(test_sampler, uv).xyz, frag_color.w);
}
