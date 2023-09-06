#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::pbr_functions as pbr_functions

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
    ambient: vec4<f32>,
    diffuse_reflectance: vec4<f32>,
    specular_reflectance: vec4<f32>,
    shininess: f32,
}
// Each wave mat3x3 is:
//  [0][0]: direction.x
//  [0][1]: direction.y
//  [0][2]: frequency
//  [1][0]: amplitude
//  [1][1]: phase
//  [1][2]: steepness
//  [2][0]: type (0 = Sine, 1 = SteepSine)
//  remainder: unused

struct WaveSpec {
    ty: f32,
    direction: vec2<f32>,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    steepness: f32,
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

fn steep_sine_wave(world_position: vec4<f32>, wave: WaveSpec) -> f32 {
	var xz: f32 = get_wave_coord(world_position, wave.direction);
	var t: f32 = get_time(wave);
	return 2.0 * wave.amplitude * pow((sin(xz * wave.frequency + t) + 1.0) / 2.0, wave.steepness);
}

fn calculate_offset(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
    var offset: f32 = 0.0;
    if wave.ty == 0.0 {
        return vec3<f32>(0.0, sine_wave(world_position, wave), 0.0);
    } else if wave.ty == 1.0 {
        return vec3<f32>(0.0, steep_sine_wave(world_position, wave), 0.0);
    }
    return vec3<f32>(0.0, offset, 0.0);
}

fn sine_normal(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
	var xz: f32 = get_wave_coord(world_position, wave.direction);
	var t: f32 = get_time(wave);
	var normal: vec2<f32> = wave.frequency * wave.amplitude * wave.direction * cos(xz * wave.frequency + t);
	return vec3<f32>(normal.x, normal.y, 0.0);
}

fn steep_sine_normal(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
	var xz: f32 = get_wave_coord(world_position, wave.direction);
	var t: f32 = get_time(wave);

	var height: f32 = pow((sin(xz * wave.frequency + t) + 1.0) / 2.0, max(1.0, wave.steepness - 1.0));
	var normal: vec2<f32> = wave.direction * wave.steepness * wave.frequency * wave.amplitude * height * cos(xz * wave.frequency + t);

	return vec3<f32>(normal.x, normal.y, 0.0);
}

fn calculate_normal(world_position: vec4<f32>, wave: WaveSpec) -> vec3<f32> {
    var normal: vec3<f32> = vec3<f32>(0.0);
    if wave.ty == 0.0 {
        normal = sine_normal(world_position, wave);
    } else if wave.ty == 1.0 {
        normal = steep_sine_normal(world_position, wave);
    }
    return normal;
}

fn get_waves() -> array<WaveSpec, WAVE_COUNT> {
    var waves = array<WaveSpec, WAVE_COUNT>();
    for (var i = 0; i < WAVE_COUNT; i++) {
        waves[i] = WaveSpec(
            material.waves[i][2][0],                                     // type
            vec2<f32>(material.waves[i][0][0], material.waves[i][0][1]), // direction
            material.waves[i][0][2],                                     // frequency
            material.waves[i][1][0],                                     // amplitude
            material.waves[i][1][1],                                     // phase
            material.waves[i][1][2]                                      // steepness
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

    var sun_direction = vec3<f32>(1.0, -1.0, 0.0);
    var sun_color = vec4<f32>(3.0, 1.9, 0.9, 1.0);

    var light_direction: vec3<f32> = -normalize(sun_direction);
    var view_direction: vec3<f32> = pbr_functions::calculate_view(mesh.world_position, false);
    var halfway_direction: vec3<f32> = normalize(light_direction + view_direction);

    var normal: vec3<f32> = vec3<f32>(0.0);
    var height: f32 = 0.0;
    for (var i = 0; i < WAVE_COUNT; i++) {
        normal += calculate_normal(mesh.world_position, waves[i]);
    }
    normal = mesh_functions::mesh_normal_local_to_world(normalize(vec3<f32>(-normal.x, 1.0, -normal.y)));

    var ndotl: f32 = saturate(dot(light_direction, normal));

    var diffuse_reflectance: vec3<f32> = material.diffuse_reflectance.xyz / PI;
    var diffuse: vec3<f32> = sun_color.rgb * ndotl * diffuse_reflectance;

    // Schlick Fresnel
    // float3 fresnel_normal = normal;
    // fresnel_normal.xz *= material.fresnel_normal_strength;
    // fresnel_normal = normalize(fresnel_normal);
    // float base = 1 - dot(viewDir, fresnelNormal);
    // float exponential = pow(base, material.fresnel_shininess);
    // float R = exponential + material.fresnel_bias * (1.0f - exponential);
    // R *= material.fresnel_strength;

    // float3 fresnel = material.fresnel_color * R;

    var specular_reflectance: vec3<f32> = material.specular_reflectance.rgb;
    var specular_normal: vec3<f32> = normal;
    var specular_amount: f32 = pow(saturate(dot(specular_normal, halfway_direction)), material.shininess * 100.0) * ndotl;
    var specular: vec3<f32> = sun_color.rgb * specular_reflectance * specular_amount;

    // Schlick Fresnel but again for specular
    // base = 1 - DotClamped(view_direction, halfway_direction);
    // exponential = pow(base, 5.0);
    // R = exponential + material.fresnel_bias * (1.0 - exponential);
    // specular *= R;

    // var tip_color: vec3<f32> = material.tip_color * pow(height, material.tip_attenuation);
    // var output: vec3<f32> = material.ambient + diffuse + specular + fresnel + tip_color;
    var output: vec3<f32> = material.ambient.rgb + diffuse + specular;
    return vec4<f32>(output, 1.0);

    // return vec4<f32>(normal.x, normal.y, normal.z, 1.0);
    // return material.diffuse_reflectance;
}
