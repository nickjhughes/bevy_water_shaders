use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts, EguiPlugin,
};
use bevy_turborand::prelude::*;

mod common;
mod fbm_water;
mod fft_water;
mod sum_water;

const PLANE_LENGTH: f32 = 100.0;
const QUAD_RES: f32 = 10.0;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
enum WaveMethod {
    SumOfSines,
    #[default]
    Fbm,
    Fft,
}

#[derive(Component, Debug)]
struct Water;

#[derive(Resource, Debug)]
struct WaterMaterials {
    sum: Handle<sum_water::SumWaterMaterial>,
    fbm: Handle<fbm_water::FbmWaterMaterial>,
    fft: Handle<fft_water::FftWaterMaterial>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RngPlugin::default(),
            EguiPlugin,
            MaterialPlugin::<sum_water::SumWaterMaterial>::default(),
            MaterialPlugin::<fbm_water::FbmWaterMaterial>::default(),
            MaterialPlugin::<fft_water::FftWaterMaterial>::default(),
        ))
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb_u8(203, 180, 152)))
        .insert_resource(sum_water::WaveType::default())
        .insert_resource(WaveMethod::default())
        .insert_resource(UiState::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_time,
                ui_system,
                update_wave_type.run_if(resource_changed::<sum_water::WaveType>()),
                update_wave_method.run_if(resource_changed::<WaveMethod>()),
                bevy::window::close_on_esc,
            ),
        )
        .add_systems(
            Update,
            ui_state_update.run_if(resource_changed::<UiState>()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    mut fbm_materials: ResMut<Assets<fbm_water::FbmWaterMaterial>>,
    mut fft_materials: ResMut<Assets<fft_water::FftWaterMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
    wave_type: Res<sum_water::WaveType>,
    wave_method: Res<WaveMethod>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(PLANE_LENGTH * 0.5, 3.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Water
    let mesh = meshes.add(
        shape::Plane {
            size: PLANE_LENGTH,
            subdivisions: (PLANE_LENGTH * QUAD_RES).round() as u32,
        }
        .into(),
    );
    let sum_water_material = sum_materials.add(sum_water::SumWaterMaterial::random(
        *wave_type,
        global_rng.as_mut(),
    ));
    let fbm_water_material = fbm_materials.add(fbm_water::FbmWaterMaterial::new());
    let fft_water_material = fft_materials.add(fft_water::FftWaterMaterial::default());
    commands.insert_resource(WaterMaterials {
        sum: sum_water_material.clone(),
        fbm: fbm_water_material.clone(),
        fft: fft_water_material.clone(),
    });
    match wave_method.as_ref() {
        WaveMethod::SumOfSines => commands.spawn((
            Water,
            MaterialMeshBundle {
                mesh,
                material: sum_water_material,
                ..default()
            },
        )),
        WaveMethod::Fbm => commands.spawn((
            Water,
            MaterialMeshBundle {
                mesh,
                material: fbm_water_material,
                ..default()
            },
        )),
        WaveMethod::Fft => commands.spawn((
            Water,
            MaterialMeshBundle {
                mesh,
                material: fft_water_material,
                ..default()
            },
        )),
    };
}

fn update_time(
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    mut fbm_materials: ResMut<Assets<fbm_water::FbmWaterMaterial>>,
    mut fft_materials: ResMut<Assets<fft_water::FftWaterMaterial>>,
    time: Res<Time>,
) {
    for material in sum_materials.iter_mut() {
        material.1.time = time.elapsed_seconds_wrapped();
    }
    for material in fbm_materials.iter_mut() {
        material.1.time = time.elapsed_seconds_wrapped();
    }
    for material in fft_materials.iter_mut() {
        material.1.time = time.elapsed_seconds_wrapped();
    }
}

fn update_wave_type(
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    wave_type: Res<sum_water::WaveType>,
) {
    for material in sum_materials.iter_mut() {
        for wave in material.1.waves.iter_mut() {
            wave.ty = *wave_type;
        }
    }
}

fn update_wave_method(
    mut commands: Commands,
    wave_method: Res<WaveMethod>,
    water_query: Query<Entity, With<Water>>,
    water_materials: Res<WaterMaterials>,
) {
    for entity in water_query.iter() {
        match *wave_method {
            WaveMethod::SumOfSines => {
                commands
                    .entity(entity)
                    .remove::<Handle<fbm_water::FbmWaterMaterial>>();
                commands
                    .entity(entity)
                    .remove::<Handle<fft_water::FftWaterMaterial>>();
                commands.entity(entity).insert(water_materials.sum.clone());
            }
            WaveMethod::Fbm => {
                commands
                    .entity(entity)
                    .remove::<Handle<sum_water::SumWaterMaterial>>();
                commands
                    .entity(entity)
                    .remove::<Handle<fft_water::FftWaterMaterial>>();
                commands.entity(entity).insert(water_materials.fbm.clone());
            }
            WaveMethod::Fft => {
                commands
                    .entity(entity)
                    .remove::<Handle<sum_water::SumWaterMaterial>>();
                commands
                    .entity(entity)
                    .remove::<Handle<fbm_water::FbmWaterMaterial>>();
                commands.entity(entity).insert(water_materials.fft.clone());
            }
        }
    }
}

#[derive(Debug)]
struct Colors {
    ambient: Color32,
    diffuse: Color32,
    specular: Color32,
    tip: Color32,
}

impl Default for Colors {
    fn default() -> Self {
        let shading_default = common::Shading::default();
        let ambient = shading_default.ambient.as_rgba_u8();
        let diffuse = shading_default.diffuse_reflectance.as_rgba_u8();
        let specular = shading_default.specular_reflectance.as_rgba_u8();
        let tip = shading_default.tip_color.as_rgba_u8();
        Colors {
            ambient: Color32::from_rgb(ambient[0], ambient[1], ambient[2]),
            diffuse: Color32::from_rgb(diffuse[0], diffuse[1], diffuse[2]),
            specular: Color32::from_rgb(specular[0], specular[1], specular[2]),
            tip: Color32::from_rgb(tip[0], tip[1], tip[2]),
        }
    }
}

#[derive(Resource, Debug, Default)]
struct UiState {
    wave_method: WaveMethod,
    wave_type: sum_water::WaveType,
    shading: common::Shading,
    colors: Colors,
    fbm_config: fbm_water::FbmWaterConfig,
}

fn ui_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    wave_method: Res<WaveMethod>,
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("settings")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Method");
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut ui_state.wave_method,
                        WaveMethod::SumOfSines,
                        "SumSines",
                    );
                    ui.radio_value(&mut ui_state.wave_method, WaveMethod::Fbm, "FBM");
                    ui.radio_value(&mut ui_state.wave_method, WaveMethod::Fft, "FFT");
                });
                ui.end_row();
            });

        // Shading
        egui::CollapsingHeader::new("Shading").show(ui, |ui| {
            egui::Grid::new("shading")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Ambient Color");
                    ui.color_edit_button_srgba(&mut ui_state.colors.ambient);
                    ui.end_row();

                    ui.label("Diffuse Color");
                    ui.color_edit_button_srgba(&mut ui_state.colors.diffuse);
                    ui.end_row();

                    ui.label("Specular Color");
                    ui.color_edit_button_srgba(&mut ui_state.colors.specular);
                    ui.end_row();

                    ui.label("Shininess");
                    ui.add(
                        egui::Slider::new(&mut ui_state.shading.shininess, 0.0..=100.0)
                            .step_by(0.5),
                    );
                    ui.end_row();

                    ui.label("Fresnel Bias");
                    ui.add(
                        egui::Slider::new(&mut ui_state.shading.fresnel.bias, 0.0..=1.0)
                            .step_by(0.01),
                    );
                    ui.end_row();

                    ui.label("Fresnel Strength");
                    ui.add(
                        egui::Slider::new(&mut ui_state.shading.fresnel.strength, 0.0..=1.0)
                            .step_by(0.01),
                    );
                    ui.end_row();

                    ui.label("Fresnel Shininess");
                    ui.add(
                        egui::Slider::new(&mut ui_state.shading.fresnel.shininess, 0.0..=20.0)
                            .step_by(0.5),
                    );
                    ui.end_row();

                    ui.label("Tip Attenuation");
                    ui.add(
                        egui::Slider::new(&mut ui_state.shading.tip_attenuation, 0.0..=10.0)
                            .step_by(0.1),
                    );
                    ui.end_row();

                    ui.label("Tip Color");
                    ui.color_edit_button_srgba(&mut ui_state.colors.tip);
                    ui.end_row();
                });
        });

        // FBM
        if *wave_method == WaveMethod::Fbm {
            egui::CollapsingHeader::new("FBM Vertex Shader").show(ui, |ui| {
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_seed, 0.0..=300.0)
                        .step_by(1.0)
                        .text("Vertex Seed"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_seed_iter, 0.0..=2000.0)
                        .step_by(10.0)
                        .text("Vertex Seed Iterator"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_frequency, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Frequency"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_frequency_mult, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Frequency Mult"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_amplitude, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Amplitude"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_amplitude_mult, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Amplitude Mult"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_max_peak, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Max Peak"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_peak_offset, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Peak Offset"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_initial_speed, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Initial Speed"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_speed_ramp, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Speed Ramp"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_drag, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Drag"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.vertex_height, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Vertex Height"),
                );
            });
            egui::CollapsingHeader::new("FBM Fragment Shader").show(ui, |ui| {
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_seed, 0.0..=300.0)
                        .step_by(1.0)
                        .text("Fragment Seed"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_seed_iter, 0.0..=2000.0)
                        .step_by(10.0)
                        .text("Fragment Seed Iterator"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_frequency, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Frequency"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_frequency_mult, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Frequency Mult"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_amplitude, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Amplitude"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_amplitude_mult, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Amplitude Mult"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_max_peak, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Max Peak"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_peak_offset, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Peak Offset"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_initial_speed, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Initial Speed"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_speed_ramp, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Speed Ramp"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_drag, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Drag"),
                );
                ui.add(
                    egui::Slider::new(&mut ui_state.fbm_config.fragment_height, 0.0..=2.0)
                        .step_by(0.1)
                        .text("Fragment Height"),
                );
            });
        }

        if *wave_method == WaveMethod::SumOfSines {
            // TODO: Add wave settings

            egui::Grid::new("sum_of_sines")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Wave Type");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut ui_state.wave_type, sum_water::WaveType::Sine, "Sine");
                        ui.radio_value(
                            &mut ui_state.wave_type,
                            sum_water::WaveType::SteepSine,
                            "Steep Sine",
                        );
                    });
                    ui.end_row();

                    ui.label("");
                    let button = ui.button("Regenerate Waves");
                    if button.clicked() {
                        for material in sum_materials.iter_mut() {
                            material.1.randomize(global_rng.as_mut());
                        }
                    }
                    ui.end_row();
                });
        }
    });
}

fn ui_state_update(
    mut commands: Commands,
    ui_state: Res<UiState>,
    mut sum_materials: ResMut<Assets<sum_water::SumWaterMaterial>>,
    mut fbm_materials: ResMut<Assets<fbm_water::FbmWaterMaterial>>,
    wave_method: Res<WaveMethod>,
    wave_type: Res<sum_water::WaveType>,
) {
    if *wave_method != ui_state.wave_method {
        commands.insert_resource(ui_state.wave_method);
    }
    if *wave_type != ui_state.wave_type {
        commands.insert_resource(ui_state.wave_type);
    }

    for material in sum_materials.iter_mut() {
        material.1.shading = ui_state.shading.clone();
        material.1.shading.ambient = Color::rgb_u8(
            ui_state.colors.ambient.r(),
            ui_state.colors.ambient.g(),
            ui_state.colors.ambient.b(),
        );
        material.1.shading.diffuse_reflectance = Color::rgb_u8(
            ui_state.colors.diffuse.r(),
            ui_state.colors.diffuse.g(),
            ui_state.colors.diffuse.b(),
        );
        material.1.shading.specular_reflectance = Color::rgb_u8(
            ui_state.colors.specular.r(),
            ui_state.colors.specular.g(),
            ui_state.colors.specular.b(),
        );
        material.1.shading.tip_color = Color::rgb_u8(
            ui_state.colors.tip.r(),
            ui_state.colors.tip.g(),
            ui_state.colors.tip.b(),
        );
    }
    for material in fbm_materials.iter_mut() {
        material.1.shading = ui_state.shading.clone();
        material.1.shading.ambient = Color::rgb_u8(
            ui_state.colors.ambient.r(),
            ui_state.colors.ambient.g(),
            ui_state.colors.ambient.b(),
        );
        material.1.shading.diffuse_reflectance = Color::rgb_u8(
            ui_state.colors.diffuse.r(),
            ui_state.colors.diffuse.g(),
            ui_state.colors.diffuse.b(),
        );
        material.1.shading.specular_reflectance = Color::rgb_u8(
            ui_state.colors.specular.r(),
            ui_state.colors.specular.g(),
            ui_state.colors.specular.b(),
        );
        material.1.shading.tip_color = Color::rgb_u8(
            ui_state.colors.tip.r(),
            ui_state.colors.tip.g(),
            ui_state.colors.tip.b(),
        );
        material.1.fbm_config = ui_state.fbm_config.clone();
    }
}
