use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
    },
};
use bevy_turborand::prelude::*;
use std::{f32::consts::PI, ops::RangeInclusive};

const WAVE_COUNT: usize = 4;
const MEDIAN_WAVELENGTH: f32 = 1.0;
const WAVELENGTH_RANGE: f32 = 1.0;
const MEDIAN_DIRECTION: f32 = 0.0;
const DIRECTIONAL_RANGE: f32 = 30.0 * PI / 180.0;
const MEDIAN_AMPLITUDE: f32 = 0.1;
const MEDIAN_SPEED: f32 = 0.5;
const SPEED_RANGE: f32 = 0.1;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub enum WaveType {
    Sine = 0,
    #[default]
    SteepSine = 1,
}

impl WaveType {
    pub fn cycle(&self) -> WaveType {
        match self {
            WaveType::Sine => WaveType::SteepSine,
            WaveType::SteepSine => WaveType::Sine,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct WaveSpec {
    pub ty: WaveType,
    direction: Vec2,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    steepness: f32,
}

impl From<WaveSpec> for Mat3 {
    fn from(wave: WaveSpec) -> Self {
        Mat3::from_cols(
            Vec3::new(wave.direction.x, wave.direction.y, wave.frequency),
            Vec3::new(wave.amplitude, wave.phase, wave.steepness),
            Vec3::new((wave.ty as u32) as f32, 0.0, 0.0),
        )
    }
}

impl Default for WaveSpec {
    fn default() -> Self {
        WaveSpec::new(WaveType::Sine, 0.0, 1.0, 1.0, 1.0, 1.0)
    }
}

impl WaveSpec {
    pub fn new(
        ty: WaveType,
        direction: f32,
        speed: f32,
        amplitude: f32,
        wavelength: f32,
        steepness: f32,
    ) -> Self {
        WaveSpec {
            ty,
            direction: Vec2::new(direction.cos(), direction.sin()),
            frequency: 2.0 / wavelength,
            amplitude,
            phase: speed * (9.8 * 2.0 * PI / wavelength).sqrt(),
            steepness,
        }
    }

    pub fn random(ty: WaveType, rng: &mut GlobalRng) -> Self {
        let wavelength = random_f32_range(
            rng,
            (MEDIAN_WAVELENGTH / (1.0 + WAVELENGTH_RANGE))
                ..=(MEDIAN_WAVELENGTH * (1.0 + WAVELENGTH_RANGE)),
        );
        let direction = random_f32_range(
            rng,
            (MEDIAN_DIRECTION - DIRECTIONAL_RANGE)..=(MEDIAN_DIRECTION + DIRECTIONAL_RANGE),
        );
        let amplitude = wavelength * (MEDIAN_AMPLITUDE / MEDIAN_WAVELENGTH);
        let speed = random_f32_range(
            rng,
            ((MEDIAN_SPEED - SPEED_RANGE).max(0.01))..=(MEDIAN_SPEED + SPEED_RANGE),
        );
        let steepness = 2.0;
        WaveSpec::new(ty, direction, speed, amplitude, wavelength, steepness)
    }
}

fn random_f32_range(rng: &mut GlobalRng, range: RangeInclusive<f32>) -> f32 {
    rng.f32() * (range.end() - range.start()) + range.start()
}

/// "Sum of Sines" based water material.
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uniform(0, WaterMaterialUniform)]
#[uuid = "d3a49f45-e0ab-49bb-bc8c-bdb020d289a6"]
pub struct SumWaterMaterial {
    pub time: f32,
    pub waves: [WaveSpec; WAVE_COUNT],
    pub shading: super::common::Shading,
}

impl SumWaterMaterial {
    pub fn random(wave_type: WaveType, rng: &mut GlobalRng) -> Self {
        let waves: [WaveSpec; WAVE_COUNT] = {
            let mut v = Vec::new();
            v.resize_with(WAVE_COUNT, || WaveSpec::random(wave_type, rng));
            v.try_into().unwrap()
        };
        SumWaterMaterial {
            time: 0.0,
            waves,
            shading: super::common::Shading::default(),
        }
    }

    pub fn randomize(&mut self, rng: &mut GlobalRng) {
        let wave_type = self.waves[0].ty;
        self.waves = {
            let mut v = Vec::new();
            v.resize_with(WAVE_COUNT, || WaveSpec::random(wave_type, rng));
            v.try_into().unwrap()
        };
    }
}

#[derive(Debug, Clone, Default, ShaderType)]
struct WaterMaterialUniform {
    time: f32,
    waves: [Mat3; WAVE_COUNT],
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

impl AsBindGroupShaderType<WaterMaterialUniform> for SumWaterMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> WaterMaterialUniform {
        WaterMaterialUniform {
            time: self.time,
            waves: self.waves.clone().map(Mat3::from),
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

impl Material for SumWaterMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/sum_water_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/sum_water_material.wgsl".into()
    }
}
