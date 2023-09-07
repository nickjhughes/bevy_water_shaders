use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Shading {
    pub ambient: Color,
    pub diffuse_reflectance: Color,
    pub specular_reflectance: Color,
    pub shininess: f32,
    pub fresnel: Fresnel,
    pub tip_color: Color,
    pub tip_attenuation: f32,
}

#[derive(Debug, Clone)]
pub struct Fresnel {
    pub color: Color,
    pub bias: f32,
    pub strength: f32,
    pub shininess: f32,
}

impl Default for Shading {
    fn default() -> Self {
        Shading {
            ambient: Color::rgba_u8(0, 43, 77, 255),
            diffuse_reflectance: Color::rgba_u8(0, 43, 77, 255),
            specular_reflectance: Color::WHITE,
            shininess: 2.0,
            fresnel: Fresnel {
                color: Color::WHITE,
                bias: 0.24,
                strength: 0.12,
                shininess: 6.7,
            },
            tip_color: Color::WHITE,
            tip_attenuation: 6.0,
        }
    }
}
