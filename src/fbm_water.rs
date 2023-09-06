use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};

pub struct FbmWaterConfig {
    vertex_wave_count: usize,
    fragment_wave_count: usize,
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

impl Default for FbmWaterConfig {
    fn default() -> Self {
        FbmWaterConfig {
            vertex_wave_count: 8,
            fragment_wave_count: 40,
            vertex_seed: 0.0,
            vertex_seed_iter: 1253.2131,
            vertex_frequency: 1.0,
            vertex_frequency_mult: 1.18,
            vertex_amplitude: 1.0,
            vertex_amplitude_mult: 0.82,
            vertex_initial_speed: 2.0,
            vertex_speed_ramp: 1.07,
            vertex_drag: 1.0,
            vertex_height: 1.0,
            vertex_max_peak: 1.0,
            vertex_peak_offset: 1.0,
            fragment_seed: 0.0,
            fragment_seed_iter: 1253.2131,
            fragment_frequency: 1.0,
            fragment_frequency_mult: 1.18,
            fragment_amplitude: 1.0,
            fragment_amplitude_mult: 0.82,
            fragment_initial_speed: 2.0,
            fragment_speed_ramp: 1.07,
            fragment_drag: 1.0,
            fragment_height: 1.0,
            fragment_max_peak: 1.0,
            fragment_peak_offset: 1.0,
        }
    }
}

/// "Fractional Brownian Motion" based water material.
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uniform(0, FbmMaterialUniform)]
#[uuid = "5f37d7f4-3403-4639-9d92-b4e5832e1514"]
pub struct FbmWaterMaterial {
    pub time: f32,
}

#[derive(Debug, Clone, Default, ShaderType)]
struct FbmMaterialUniform {
    time: f32,
}

impl AsBindGroupShaderType<FbmMaterialUniform> for FbmWaterMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> FbmMaterialUniform {
        FbmMaterialUniform { time: self.time }
    }
}

impl Material for FbmWaterMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/fbm_water_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/fbm_water_material.wgsl".into()
    }
}
