#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_bindings mesh

const PI: f32 = 3.1415926538;
const WAVE_COUNT: i32 = 4;

struct MeshVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
}

struct WaterMaterial {
    time: f32,
    waves: array<mat3x3<f32>, WAVE_COUNT>,
}
// Each wave mat3x3 is:
//  0: direction.x
//  1: direction.y
//  2: frequency
//  3: amplitude
//  4: phase
//  5-8: unused

struct WaveSpec {
    direction: vec2<f32>,
    frequency: f32,
    amplitude: f32,
    phase: f32,
}

@group(1) @binding(0)
var<uniform> material: WaterMaterial;

struct Vertex {
    @location(0) position: vec3<f32>,
};

fn get_wave_coord(world_position: vec4<f32>, direction: vec2<f32>) -> f32 {
    return world_position.x * direction.x + world_position.z * direction.y;
}

fn get_time(wave: WaveSpec) -> f32 {
    return material.time * wave.phase;
}

fn sine_wave(world_position: vec4<f32>, wave: WaveSpec) -> f32 {
	var xz: f32 = get_wave_coord(world_position, wave.direction);
	var t: f32 = get_time(wave);

	return wave.amplitude * sin(xz * wave.frequency + t);
}

fn calculate_offset(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
    return vec3<f32>(0.0, sine_wave(world_position, wave), 0.0);
}

fn sine_normal(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
	var xz: f32 = get_wave_coord(world_position, wave.direction);
	var t: f32 = get_time(wave);
	var n: vec2<f32> = wave.frequency * wave.amplitude * wave.direction * cos(xz * wave.frequency + t);
	return vec3<f32>(n.x, n.y, 0.0);
}

fn calculate_normal(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
    return sine_normal(world_position, wave);
}

fn get_waves() -> array<WaveSpec, WAVE_COUNT> {
    var waves = array<WaveSpec, WAVE_COUNT>();
    for (var i = 0; i < WAVE_COUNT; i++) {
        waves[i] = WaveSpec(
            vec2<f32>(material.waves[i][0][0], material.waves[i][0][1]), // direction
            material.waves[i][0][2],                                     // frequency
            material.waves[i][1][0],                                     // amplitude
            material.waves[i][1][1]                                      // phase
        );
    }
    return waves;
}

@vertex
fn vertex(vertex: Vertex) -> MeshVertexOutput {
    var waves = get_waves();

    var base_world_position = mesh_functions::mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
    var offset: vec3<f32> = vec3<f32>(0.0);
	for (var i: i32 = 0; i < WAVE_COUNT; i++) {
	   offset += calculate_offset(base_world_position, waves[i]);
	}
	var offset_position = vertex.position + offset;

    var out: MeshVertexOutput;
	out.world_position = mesh_functions::mesh_position_local_to_world(mesh.model, vec4<f32>(offset_position, 1.0));
	out.position = mesh_functions::mesh_position_world_to_clip(out.world_position);
    out.world_normal = vec3<f32>(0.0, 0.0, 0.0);
    return out;
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    var waves = get_waves();

    // var light_direction: vec3<f32> = -normalize(material.sun_direction);
    // var view_direction: vec3<f32> = normalize(_WorldSpaceCameraPos - i.worldPos);
    // var halfway_direction: vec3<f32> = normalize(light_direction + view_direction);

    var normal: vec3<f32> = vec3<f32>(0.0);
    var height: f32 = 0.0;
    for (var i = 0; i < WAVE_COUNT; i++) {
        normal += calculate_normal(mesh.world_position, waves[i]);
    }
    normal = mesh_functions::mesh_normal_local_to_world(normalize(vec3<f32>(-normal.x, 1.0, -normal.y)));

    // var ndotl: f32 = saturate(dot(light_direction, normal));

    // var diffuse_reflectance: f32 = matieral.diffuse_reflectance / PI;
    // var diffuse: f32 = material.light_color0.rgb * ndotl * diffuse_reflectance;

    // Schlick Fresnel
    // float3 fresnel_normal = normal;
    // fresnel_normal.xz *= material.fresnel_normal_strength;
    // fresnel_normal = normalize(fresnel_normal);
    // float base = 1 - dot(viewDir, fresnelNormal);
    // float exponential = pow(base, material.fresnel_shininess);
    // float R = exponential + material.fresnel_bias * (1.0f - exponential);
    // R *= material.fresnel_strength;

    // float3 fresnel = material.fresnel_color * R;

    // float3 specularReflectance = _SpecularReflectance;
    // float3 specNormal = normal;
    // specNormal.xz *= _SpecularNormalStrength;
    // specNormal = normalize(specNormal);
    // float spec = pow(DotClamped(specNormal, halfwayDir), _Shininess) * ndotl;
    // float3 specular = _LightColor0.rgb * specularReflectance * spec;

    // Schlick Fresnel but again for specular
    // base = 1 - DotClamped(view_direction, halfway_direction);
    // exponential = pow(base, 5.0);
    // R = exponential + material.fresnel_bias * (1.0 - exponential);
    // specular *= R;

    // var tip_color: vec3<f32> = material.tip_color * pow(height, material.tip_attenuation);
    // var output: vec3<f32> = material.ambient + diffuse + specular + fresnel + tip_color;
    // return vec4<f32>(output, 1.0);

    return vec4<f32>(normal.x, normal.y, normal.z, 1.0);
}
