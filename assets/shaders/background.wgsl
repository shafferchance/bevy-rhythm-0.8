#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(2) @binding(0)
var<uniform> mesh: Mesh;

#import bevy_pbr::mesh_functions

struct VertexInputs {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: VertexInputs) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.world_position = vec4<f32>(vertex.position, 1.0);
    out.world_normal = vertex.normal;
    out.uv = vertex.uv;
    return out;
}

struct MaterialData {
    time: f32
};

@group(1) @binding(0)
var<uniform> material_data: MaterialData;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv, abs(sin(material_data.time)), 1.0);
}
