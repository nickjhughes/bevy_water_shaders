#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::pbr_functions as pbr_functions

const PI: f32 = 3.1415926538;

struct MeshVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
}

struct WaterMaterial {
    time: f32,
    ambient: vec4<f32>,
    diffuse_reflectance: vec4<f32>,
    specular_reflectance: vec4<f32>,
    shininess: f32,
    fresnel_color: vec4<f32>,
    fresnel_bias: f32,
    fresnel_strength: f32,
    fresnel_shininess: f32,
    tip_attenuation: f32,
    tip_color: vec4<f32>,
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
    var sun_direction = vec3<f32>(1.0, -1.0, 0.0);
    var sun_color = vec4<f32>(3.0, 1.9, 0.9, 1.0);

    var light_direction: vec3<f32> = -normalize(sun_direction);
    var view_direction: vec3<f32> = pbr_functions::calculate_view(mesh.world_position, false);
    var halfway_direction: vec3<f32> = normalize(light_direction + view_direction);

    var height: f32 = 0.0;
    var normal: vec3<f32> = vec3<f32>(0.0, 1.0, 0.0);

    var ndotl: f32 = saturate(dot(light_direction, normal));

    var diffuse_reflectance: vec3<f32> = material.diffuse_reflectance.xyz / PI;
    var diffuse: vec3<f32> = sun_color.rgb * ndotl * diffuse_reflectance;

    // Schlick Fresnel
    var fresnel_normal: vec3<f32> = normal;
    // fresnel_normal.x *= material.fresnel_normal_strength;
    // fresnel_normal.z *= material.fresnel_normal_strength;
    // fresnel_normal = normalize(fresnel_normal);
    var base: f32 = 1.0 - dot(view_direction, fresnel_normal);
    var exponential: f32 = pow(base, material.fresnel_shininess);
    var R: f32 = exponential + material.fresnel_bias * (1.0 - exponential);
    R *= material.fresnel_strength;
    var fresnel: vec3<f32> = material.fresnel_color.rgb * R;

    var specular_reflectance: vec3<f32> = material.specular_reflectance.rgb;
    var specular_normal: vec3<f32> = normal;
    var specular_amount: f32 = pow(saturate(dot(specular_normal, halfway_direction)), material.shininess * 100.0) * ndotl;
    var specular: vec3<f32> = sun_color.rgb * specular_reflectance * specular_amount;

    // Schlick Fresnel but again for specular
    base = 1.0 - saturate(dot(view_direction, halfway_direction));
    exponential = pow(base, 5.0);
    R = exponential + material.fresnel_bias * (1.0 - exponential);
    specular *= R;

    var tip_color: vec3<f32> = material.tip_color.rgb * pow(height, material.tip_attenuation);

    var output: vec3<f32> = material.ambient.rgb + diffuse + specular + fresnel + tip_color;
    return vec4<f32>(output, 1.0);
}
