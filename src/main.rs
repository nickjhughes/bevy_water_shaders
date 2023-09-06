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

const PLANE_LENGTH: f32 = 100.0;
const QUAD_RES: f32 = 10.0;

const WAVE_COUNT: usize = 4;
const MEDIAN_WAVELENGTH: f32 = 8.0;
const WAVELENGTH_RANGE: f32 = 1.0;
const MEDIAN_DIRECTION: f32 = 0.0;
const DIRECTIONAL_RANGE: f32 = 30.0 * PI / 180.0;
const MEDIAN_AMPLITUDE: f32 = 1.0;
const MEDIAN_SPEED: f32 = 1.0;
const SPEED_RANGE: f32 = 0.1;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RngPlugin::default(),
            MaterialPlugin::<WaterMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_time,
                rotate_camera,
                regenerate_waves,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(PLANE_LENGTH, PLANE_LENGTH * 0.7, PLANE_LENGTH)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Water
    let water_material = WaterMaterial::random(global_rng.as_mut());
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            shape::Plane {
                size: PLANE_LENGTH,
                subdivisions: (PLANE_LENGTH * QUAD_RES).round() as u32,
            }
            .into(),
        ),
        material: materials.add(water_material),
        ..default()
    });

    // Sun
    // commands.spawn((
    //     SpatialBundle {
    //         transform: Transform::from_xyz(0.0, 10.0, 0.0),
    //         ..default()
    //     },
    //     Sun {
    //         color: Color::WHITE,
    //         direction: Vec3::Y,
    //     },
    // ));
}

fn update_time(mut materials: ResMut<Assets<WaterMaterial>>, time: Res<Time>) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds_wrapped();
    }
}

fn rotate_camera(mut camera_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    const CAMERA_ROTATION_SPEED: f32 = 0.1;
    let mut camera_transform = camera_query.single_mut();
    *camera_transform = Transform::from_xyz(
        1.2 * PLANE_LENGTH * (time.elapsed_seconds_wrapped() * CAMERA_ROTATION_SPEED).cos(),
        camera_transform.translation.y,
        1.2 * PLANE_LENGTH * (time.elapsed_seconds_wrapped() * CAMERA_ROTATION_SPEED).sin(),
    )
    .looking_at(Vec3::ZERO, Vec3::Y);
}

fn regenerate_waves(
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for material in materials.iter_mut() {
            material.1.randomize(global_rng.as_mut());
        }
    }
}

// #[derive(Component)]
// struct Sun {
//     color: Color,
//     direction: Vec3,
// }

#[derive(Component, Debug, Clone)]
struct WaveSpec {
    direction: Vec2,
    frequency: f32,
    amplitude: f32,
    phase: f32,
}

impl From<WaveSpec> for Mat3 {
    fn from(wave: WaveSpec) -> Self {
        Mat3::from_cols(
            Vec3::new(wave.direction.x, wave.direction.y, wave.frequency),
            Vec3::new(wave.amplitude, wave.phase, 0.0),
            Vec3::ZERO,
        )
    }
}

impl Default for WaveSpec {
    fn default() -> Self {
        WaveSpec::new(0.0, 1.0, 1.0, 1.0)
    }
}

impl WaveSpec {
    fn new(direction: f32, speed: f32, amplitude: f32, wavelength: f32) -> Self {
        WaveSpec {
            direction: Vec2::new(direction.cos(), direction.sin()),
            frequency: 2.0 / wavelength,
            amplitude,
            phase: speed * (9.8 * 2.0 * PI / wavelength).sqrt(),
        }
    }

    fn random(rng: &mut GlobalRng) -> Self {
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
        WaveSpec::new(direction, speed, amplitude, wavelength)
    }
}

fn random_f32_range(rng: &mut GlobalRng, range: RangeInclusive<f32>) -> f32 {
    rng.f32() * (range.end() - range.start()) + range.start()
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uniform(0, WaterMaterialUniform)]
#[uuid = "d3a49f45-e0ab-49bb-bc8c-bdb020d289a6"]
struct WaterMaterial {
    time: f32,
    waves: [WaveSpec; WAVE_COUNT],
    ambient: Color,
    diffuse_reflectance: Color,
    specular_reflectance: Color,
    shininess: f32,
    // fresnel: Fresnel,
    // tip_color: Color,
    // tip_attenuation: f32,
}

// #[derive(Debug, Clone)]
// struct Fresnel {
//     color: Color,
//     bias: f32,
//     strength: f32,
//     shininess: f32,
// }

impl WaterMaterial {
    fn random(rng: &mut GlobalRng) -> Self {
        let waves: [WaveSpec; WAVE_COUNT] = {
            let mut v = Vec::new();
            v.resize_with(WAVE_COUNT, || WaveSpec::random(rng));
            v.try_into().unwrap()
        };
        WaterMaterial {
            time: 0.0,
            ambient: Color::rgba_u8(0, 43, 77, 255),
            diffuse_reflectance: Color::rgba_u8(0, 43, 77, 255),
            specular_reflectance: Color::WHITE,
            shininess: 1.0,
            waves,
        }
    }

    fn randomize(&mut self, rng: &mut GlobalRng) {
        self.waves = {
            let mut v = Vec::new();
            v.resize_with(WAVE_COUNT, || WaveSpec::random(rng));
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
}

impl AsBindGroupShaderType<WaterMaterialUniform> for WaterMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> WaterMaterialUniform {
        WaterMaterialUniform {
            time: self.time,
            waves: self.waves.clone().map(Mat3::from),
            ambient: self.ambient,
            diffuse_reflectance: self.diffuse_reflectance,
            specular_reflectance: self.specular_reflectance,
            shininess: self.shininess,
        }
    }
}

impl Material for WaterMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/water_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/water_material.wgsl".into()
    }
}
