use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};
use std::f32::consts::PI;

const GRAVITY: f32 = 9.81;

#[derive(Debug, Clone)]
struct SpectrumSettings {
    pub scale: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub fetch: f32,
    pub spread_blend: f32,
    pub swell: f32,
    pub peak_enhancement: f32,
    pub short_waves_fade: f32,
}

#[derive(Debug, Clone)]
struct SpectrumSettingsUniform {
    pub scale: f32,
    pub angle: f32,
    pub spread_blend: f32,
    pub swell: f32,
    pub alpha: f32,
    pub peak_omega: f32,
    pub gamma: f32,
    pub short_waves_fade: f32,
}

fn jonswap_alpha(fetch: f32, wind_speed: f32) -> f32 {
    0.076 * (GRAVITY * fetch / wind_speed / wind_speed).powf(-0.22)
}

fn jonswap_peak_frequency(fetch: f32, wind_speed: f32) -> f32 {
    22.0 * (wind_speed * fetch / GRAVITY / GRAVITY).powf(-0.33)
}

impl SpectrumSettingsUniform {
    pub fn from_spectrum_settings(settings: &SpectrumSettings) -> Self {
        SpectrumSettingsUniform {
            scale: settings.scale,
            angle: settings.wind_direction / 180.0 * PI,
            spread_blend: settings.spread_blend,
            swell: settings.swell.clamp(0.01, 1.0),
            alpha: jonswap_alpha(settings.fetch, settings.wind_speed),
            peak_omega: jonswap_peak_frequency(settings.fetch, settings.wind_speed),
            gamma: settings.peak_enhancement,
            short_waves_fade: settings.short_waves_fade,
        }
    }
}

impl Default for SpectrumSettings {
    fn default() -> Self {
        SpectrumSettings {
            scale: 0.5,
            wind_speed: 20.0,
            wind_direction: 22.0,
            fetch: 100000000.0,
            spread_blend: 1.0,
            swell: 0.42,
            peak_enhancement: 1.0,
            short_waves_fade: 1.0,
        }
    }
}

/// "Fourier Transform" based water material.
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uniform(0, WaterMaterialUniform)]
#[uuid = "e90e7bbc-912b-4f10-8088-a4c7e46b9d10"]
pub struct FftWaterMaterial {
    pub time: f32,
    pub shading: super::common::Shading,
}

#[derive(Debug, Clone, Default, ShaderType)]
struct WaterMaterialUniform {
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
}

impl AsBindGroupShaderType<WaterMaterialUniform> for FftWaterMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> WaterMaterialUniform {
        WaterMaterialUniform {
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
        }
    }
}

impl Material for FftWaterMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/fft_water_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/fft_water_material.wgsl".into()
    }
}
