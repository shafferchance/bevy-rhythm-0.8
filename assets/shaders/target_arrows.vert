#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 1) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 2, binding = 0) uniform Mesh {
    mat4 Model;
};

void main() {
    v_Uv = Vertex_Uv;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}