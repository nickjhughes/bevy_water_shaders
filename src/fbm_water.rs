use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};

#[derive(Debug, Clone)]
pub struct FbmWaterConfig {
    // Vertex shader
    pub vertex_wave_count: usize,
    pub vertex_seed: f32,
    pub vertex_seed_iter: f32,
    pub vertex_frequency: f32,
    pub vertex_frequency_mult: f32,
    pub vertex_amplitude: f32,
    pub vertex_amplitude_mult: f32,
    pub vertex_initial_speed: f32,
    pub vertex_speed_ramp: f32,
    pub vertex_drag: f32,
    pub vertex_height: f32,
    pub vertex_max_peak: f32,
    pub vertex_peak_offset: f32,
    // Fragment shader
    pub fragment_wave_count: usize,
    pub fragment_seed: f32,
    pub fragment_seed_iter: f32,
    pub fragment_frequency: f32,
    pub fragment_frequency_mult: f32,
    pub fragment_amplitude: f32,
    pub fragment_amplitude_mult: f32,
    pub fragment_initial_speed: f32,
    pub fragment_speed_ramp: f32,
    pub fragment_drag: f32,
    pub fragment_height: f32,
    pub fragment_max_peak: f32,
    pub fragment_peak_offset: f32,
}

impl Default for FbmWaterConfig {
    fn default() -> Self {
        FbmWaterConfig {
            vertex_wave_count: 40,
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
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uniform(0, FbmMaterialUniform)]
#[uuid = "5f37d7f4-3403-4639-9d92-b4e5832e1514"]
pub struct FbmWaterMaterial {
    pub time: f32,
    pub fbm_config: FbmWaterConfig,
    pub shading: super::common::Shading,
}

impl FbmWaterMaterial {
    pub fn new() -> Self {
        FbmWaterMaterial::default()
    }
}

#[derive(Debug, Clone, Default, ShaderType)]
struct FbmMaterialUniform {
    time: f32,
    ambient: Color,
    diffuse_reflectance: Color,
    specular_reflectance: Color,
    shininess: f32,
    fresnel_color: Color,
    fresnel_bias: f32,
    fresnel_strength: f32,
    fresnel_shininess: f32,
    tip_attenuation: f32,
    tip_color: Color,
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

impl AsBindGroupShaderType<FbmMaterialUniform> for FbmWaterMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> FbmMaterialUniform {
        FbmMaterialUniform {
            time: self.time,
            ambient: self.shading.ambient,
            diffuse_reflectance: self.shading.diffuse_reflectance,
            specular_reflectance: self.shading.specular_reflectance,
            shininess: self.shading.shininess,
            fresnel_color: self.shading.fresnel.color,
            fresnel_bias: self.shading.fresnel.bias,
            fresnel_strength: self.shading.fresnel.strength,
            fresnel_shininess: self.shading.fresnel.shininess,
            tip_attenuation: self.shading.tip_attenuation,
            tip_color: self.shading.tip_color,
            vertex_wave_count: self.fbm_config.vertex_wave_count as u32,
            vertex_seed: self.fbm_config.vertex_seed,
            vertex_seed_iter: self.fbm_config.vertex_seed_iter,
            vertex_frequency: self.fbm_config.vertex_frequency,
            vertex_frequency_mult: self.fbm_config.vertex_frequency_mult,
            vertex_amplitude: self.fbm_config.vertex_amplitude,
            vertex_amplitude_mult: self.fbm_config.vertex_amplitude_mult,
            vertex_initial_speed: self.fbm_config.vertex_initial_speed,
            vertex_speed_ramp: self.fbm_config.vertex_speed_ramp,
            vertex_drag: self.fbm_config.vertex_drag,
            vertex_height: self.fbm_config.vertex_height,
            vertex_max_peak: self.fbm_config.vertex_max_peak,
            vertex_peak_offset: self.fbm_config.vertex_peak_offset,
            fragment_wave_count: self.fbm_config.fragment_wave_count as u32,
            fragment_seed: self.fbm_config.fragment_seed,
            fragment_seed_iter: self.fbm_config.fragment_seed_iter,
            fragment_frequency: self.fbm_config.fragment_frequency,
            fragment_frequency_mult: self.fbm_config.fragment_frequency_mult,
            fragment_amplitude: self.fbm_config.fragment_amplitude,
            fragment_amplitude_mult: self.fbm_config.fragment_amplitude_mult,
            fragment_initial_speed: self.fbm_config.fragment_initial_speed,
            fragment_speed_ramp: self.fbm_config.fragment_speed_ramp,
            fragment_drag: self.fbm_config.fragment_drag,
            fragment_height: self.fbm_config.fragment_height,
            fragment_max_peak: self.fbm_config.fragment_max_peak,
            fragment_peak_offset: self.fbm_config.fragment_peak_offset,
        }
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
