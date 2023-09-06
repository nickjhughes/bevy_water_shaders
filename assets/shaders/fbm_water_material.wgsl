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
    vertex_wave_count: u32,
    vertex_seed: f32,
    vertex_seed_iter: f32,
    vertex_frequency: f32,
    vertex_frequency_mult: f32,
    vertex_amplitude: f32,
    vertex_amplitude_mult: f32,
    vertex_initial_speed: f32,
    vertex_speed_ramp: f32,
    vertex_drag: f32,
    vertex_height: f32,
    vertex_max_peak: f32,
    vertex_peak_offset: f32,
    fragment_wave_count: u32,
    fragment_seed: f32,
    fragment_seed_iter: f32,
    fragment_frequency: f32,
    fragment_frequency_mult: f32,
    fragment_amplitude: f32,
    fragment_amplitude_mult: f32,
    fragment_initial_speed: f32,
    fragment_speed_ramp: f32,
    fragment_drag: f32,
    fragment_height: f32,
    fragment_max_peak: f32,
    fragment_peak_offset: f32,
}

@group(1) @binding(0)
var<uniform> material: WaterMaterial;

struct Vertex {
    @location(0) position: vec3<f32>,
};

fn vertex_fbm(world_position: vec4<f32>) -> f32 {
    var frequency: f32 = material.vertex_frequency;
    var amplitude: f32 = material.vertex_amplitude;
    var speed: f32 = material.vertex_initial_speed;
    var seed: f32 = material.vertex_seed;
    var position: vec3<f32> = world_position.xyz;
    var amplitude_sum: f32 = 0.0;

    var height: f32 = 0.0;
    for (var i: u32 = u32(0); i < material.vertex_wave_count; i++) {
        var direction: vec2<f32> = normalize(vec2<f32>(cos(seed), sin(seed)));

        var x = dot(direction, position.xz) * frequency + material.time * speed;
        var wave = amplitude * exp(material.vertex_max_peak * sin(x) - material.vertex_peak_offset);

        height += wave;

        var dx = material.vertex_max_peak * wave * cos(x);
        position.x += direction.x * -dx * amplitude * material.vertex_drag;
        position.z += direction.y * -dx * amplitude * material.vertex_drag;

        amplitude_sum += 1.0;
        frequency *= material.vertex_frequency_mult;
        amplitude *= material.vertex_amplitude_mult;
        speed *= material.vertex_speed_ramp;
        seed += material.vertex_seed_iter;
	}

	var output: f32 = (height / amplitude_sum) * material.vertex_height;
	return output;
}

fn fragment_fbm(world_position: vec4<f32>) -> vec3<f32> {
    var frequency: f32 = material.fragment_frequency;
    var amplitude: f32 = material.fragment_amplitude;
    var speed: f32 = material.fragment_initial_speed;
    var seed: f32 = material.fragment_seed;
	var position: vec3<f32> = world_position.xyz;
	var amplitude_sum: f32 = 0.0;

	var height: f32 = 0.0;
	var normal: vec2<f32> = vec2<f32>(0.0);

	for (var i = u32(0); i < material.fragment_wave_count; i++) {
	    var direction: vec2<f32> = normalize(vec2<f32>(cos(seed), sin(seed)));

		var x: f32 = dot(direction, position.xz) * frequency + material.time * speed;
		var wave: f32 = amplitude * exp(material.fragment_max_peak * sin(x) - material.fragment_peak_offset);
		var dw: vec2<f32> = frequency * direction * (material.fragment_max_peak * wave * cos(x));

		height += wave;
		position.x += -dw.x * amplitude * material.fragment_drag;
	    position.z += -dw.y * amplitude * material.fragment_drag;

		normal += dw;

		amplitude_sum += amplitude;
		frequency *= material.fragment_frequency_mult;
		amplitude *= material.fragment_amplitude_mult;
		speed *= material.fragment_speed_ramp;
		seed += material.fragment_seed_iter;
	}

	var output: vec3<f32> = vec3<f32>(height, normal.x, normal.y) / amplitude_sum;
	output.x *= material.fragment_height;

	return output;
}

@vertex
fn vertex(vertex: Vertex) -> MeshVertexOutput {
    var base_world_position = mesh_functions::mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
	var offset: vec3<f32> = vec3<f32>(0.0);
	var normal: vec3<f32> = vec3<f32>(0.0);

    var fbm: f32 = vertex_fbm(base_world_position);
	offset.y = fbm;

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
    var sun_direction = vec3<f32>(1.0, -1.0, 1.0);
    var sun_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    var light_direction: vec3<f32> = -normalize(sun_direction);
    var view_direction: vec3<f32> = pbr_functions::calculate_view(mesh.world_position, false);
    var halfway_direction: vec3<f32> = normalize(light_direction + view_direction);

    var normal: vec3<f32> = vec3<f32>(0.0);
    var height: f32 = 0.0;

   	var fbm: vec3<f32> = fragment_fbm(mesh.world_position);
   	height = fbm.x;
   	normal.x = fbm.y;
    normal.y = fbm.z;
    normal = mesh_functions::mesh_normal_local_to_world(normalize(vec3<f32>(-normal.x, 1.0, -normal.y)));

    var ndotl: f32 = saturate(dot(light_direction, normal));

    var diffuse_reflectance: vec3<f32> = material.diffuse_reflectance.xyz / PI;
    var diffuse: vec3<f32> = sun_color.rgb * ndotl * diffuse_reflectance;

    var specular_reflectance: vec3<f32> = material.specular_reflectance.rgb;
    var specular_normal: vec3<f32> = normal;
    var specular_amount: f32 = pow(saturate(dot(specular_normal, halfway_direction)), material.shininess * 100.0) * ndotl;
    var specular: vec3<f32> = sun_color.rgb * specular_reflectance * specular_amount;

    var output: vec3<f32> = material.ambient.rgb + diffuse + specular;
    return vec4<f32>(output, 1.0);
}
