#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::pbr_functions as pbr_functions

// const PI: f32 = 3.1415926538;

struct MeshVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
}

struct WaterMaterial {
    time: f32,
}

@group(1) @binding(0)
var<uniform> material: WaterMaterial;

struct Vertex {
    @location(0) position: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> MeshVertexOutput {
    var out: MeshVertexOutput;
	out.world_position = mesh_functions::mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
	out.position = mesh_functions::mesh_position_world_to_clip(out.world_position);
    out.world_normal = vec3<f32>(0.0, 0.0, 0.0);
    return out;
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 1.0, 1.0);
}
