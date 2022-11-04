#version 450

layout(location = 2) in vec2 v_Uv;
layout(set = 1, binding = 0) uniform BackgroundMaterial {
    vec4 color;
    float time;
} mat;

layout(set = 1, binding = 1) uniform texture2D texture_2D;
layout(set = 1, binding = 2) uniform sampler our_sampler;

layout(location = 0) out vec4 o_Target;

void main() {
    vec4 output_color = color * vec4(time, 1.0, 1.0, 1.0);
    o_Target = output_color * texture(sampler2D(texture_2D, our_sampler), v_Uv);
}
