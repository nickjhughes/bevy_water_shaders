use bevy::prelude::*;
use bevy_turborand::prelude::*;

mod fbm_water;
mod sum_water;

const PLANE_LENGTH: f32 = 100.0;
const QUAD_RES: f32 = 10.0;

#[derive(Resource, Debug, Clone, Copy)]
enum WaveMethod {
    SumOfSines,
    Fbm,
}

impl WaveMethod {
    fn cycle(&self) -> WaveMethod {
        match self {
            WaveMethod::SumOfSines => WaveMethod::Fbm,
            WaveMethod::Fbm => WaveMethod::SumOfSines,
        }
    }
}

#[derive(Component, Debug)]
struct Water;

#[derive(Resource, Debug)]
struct WaterMaterials {
    sum: Handle<sum_water::SumWaterMaterial>,
    fbm: Handle<fbm_water::FbmWaterMaterial>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RngPlugin::default(),
            MaterialPlugin::<sum_water::SumWaterMaterial>::default(),
            MaterialPlugin::<fbm_water::FbmWaterMaterial>::default(),
        ))
        .insert_resource(sum_water::WaveType::SteepSine)
        .insert_resource(WaveMethod::SumOfSines)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_time,
                rotate_camera,
                regenerate_waves,
                change_wave_type,
                change_wave_method,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    mut fbm_materials: ResMut<Assets<fbm_water::FbmWaterMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
    wave_type: Res<sum_water::WaveType>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(PLANE_LENGTH, PLANE_LENGTH * 0.7, PLANE_LENGTH)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Water
    let sum_water_material = sum_materials.add(sum_water::SumWaterMaterial::random(
        *wave_type,
        global_rng.as_mut(),
    ));
    let fbm_water_material = fbm_materials.add(fbm_water::FbmWaterMaterial { time: 0.0 });
    commands.insert_resource(WaterMaterials {
        sum: sum_water_material.clone(),
        fbm: fbm_water_material,
    });
    commands.spawn((
        Water,
        MaterialMeshBundle {
            mesh: meshes.add(
                shape::Plane {
                    size: PLANE_LENGTH,
                    subdivisions: (PLANE_LENGTH * QUAD_RES).round() as u32,
                }
                .into(),
            ),
            material: sum_water_material,
            ..default()
        },
    ));
}

fn update_time(mut materials: ResMut<Assets<sum_water::SumWaterMaterial>>, time: Res<Time>) {
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
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        for material in sum_materials.iter_mut() {
            material.1.randomize(global_rng.as_mut());
        }
    }
}

fn change_wave_type(
    mut commands: Commands,
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    wave_type: Res<sum_water::WaveType>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::W) {
        let new_wave_type = wave_type.as_ref().cycle();
        commands.insert_resource(new_wave_type);

        for material in sum_materials.iter_mut() {
            for wave in material.1.waves.iter_mut() {
                wave.ty = new_wave_type;
            }
        }
    }
}

fn change_wave_method(
    mut commands: Commands,
    wave_method: Res<WaveMethod>,
    keyboard_input: Res<Input<KeyCode>>,
    water_query: Query<Entity, With<Water>>,
    water_materials: Res<WaterMaterials>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        let new_wave_method = wave_method.as_ref().cycle();
        commands.insert_resource(new_wave_method);

        for entity in water_query.iter() {
            match &new_wave_method {
                WaveMethod::SumOfSines => {
                    commands
                        .entity(entity)
                        .remove::<Handle<fbm_water::FbmWaterMaterial>>();
                    commands.entity(entity).insert(water_materials.sum.clone());
                }
                WaveMethod::Fbm => {
                    commands
                        .entity(entity)
                        .remove::<Handle<sum_water::SumWaterMaterial>>();
                    commands.entity(entity).insert(water_materials.fbm.clone());
                }
            }
        }
    }
}
