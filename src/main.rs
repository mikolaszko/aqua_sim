use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;

//.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins((
            DefaultPickingPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_physics)
        .add_systems(Startup, setup_cameras)
        .add_systems(Startup, setup_light)
        .add_systems(Update, setup_ui)
        .insert_resource(DebugPickingMode::Normal)
        .run();
}

fn setup_light(mut commands: Commands) {
    //Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 400_000.0,
            ..default()
        },
        transform: Transform::from_xyz(3., 7.5, -2.0),
        ..default()
    });
    //Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 400_000.0,
            ..default()
        },
        transform: Transform::from_xyz(-3., 7.5, 2.0),
        ..default()
    });
}

fn setup_cameras(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera {
            allow_upside_down: false,
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::ShiftLeft),
            ..default()
        },
    ));
}
fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Ground
    commands
        .spawn(Collider::cuboid(5.0, 0.1, 5.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.4, 0.0)));

    /* Create the bouncing ball. */
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Collider::ball(0.5),
        RigidBody::Dynamic,
        Restitution::coefficient(0.7),
        PickableBundle::default(),
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
        //TODO: figure out z position
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x / 50.0; // Make the square follow the mouse
            transform.translation.y -= drag.delta.y / 50.0; // Make the square follow the mouse
            println!("{:?}", drag)
        }),
    ));
}

#[derive(Default, Resource)]
struct UiState {
    label: String,
    is_dark_mode: bool,
}

fn setup_ui(mut ui_state: ResMut<UiState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.checkbox(&mut ui_state.is_dark_mode, "Dark Mode");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
