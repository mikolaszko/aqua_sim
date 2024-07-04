use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;

mod sand;
mod ui;

#[derive(Resource, Default, Clone, Debug)]
struct GroundCoords {
    global: Vec3,
}

fn main() {
    App::new()
        .init_resource::<GroundCoords>()
        .add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins((
            DefaultPickingPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(ui::UiPlugin)
        .add_plugins(sand::SandPlugin)
        .add_systems(Startup, setup_background)
        .add_systems(Startup, setup_cameras)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_balls)
        .add_systems(Update, cursor_to_ground_plane)
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

/// Used to help identify our main camera
#[derive(Component)]
struct MainGameCamera;

fn setup_cameras(mut commands: Commands) {
    // Camera
    commands.spawn((
        MainGameCamera,
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

/// Used to help identify our ground plane
#[derive(Component)]
struct MyGroundPlane;

fn setup_background(mut commands: Commands) {
    //Ground
    commands
        .spawn((MyGroundPlane, Collider::cuboid(5.0, 0.1, 5.0)))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.4, 0.0)));
}

fn setup_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
    ));
}

/*
On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // disable picking
On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
On::<Pointer<Drag>>::target_component_mut::<Transform>(move |_, transform| {
    transform.translation.x = mut_coords.global.x;
    transform.translation.y = mut_coords.global.y;
    transform.translation.z = mut_coords.global.z;
    print!("{:?}", mut_coords)
}),
*/

fn cursor_to_ground_plane(
    mut mycoords: ResMut<GroundCoords>,
    // query to get the window (so we can read the current cursor position)
    // (we will only work with the primary window)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainGameCamera>>,
    // query to get ground plane's transform
    q_plane: Query<&GlobalTransform, With<MyGroundPlane>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // Ditto for the ground plane's transform
    let ground_transform = q_plane.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    let Some(cursor_position) = window.cursor_position() else {
        // if the cursor is not inside the window, we can't do anything
        return;
    };

    // Mathematically, we can represent the ground as an infinite flat plane.
    // To do that, we need a point (to position the plane) and a normal vector
    // (the "up" direction, perpendicular to the ground plane).

    // We can get the correct values from the ground entity's GlobalTransform
    let plane_origin = ground_transform.translation();
    let plane = Plane3d::new(ground_transform.up());

    // Ask Bevy to give us a ray pointing from the viewport (screen) into the world
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        // if it was impossible to compute for whatever reason; we can't do anything
        return;
    };

    // do a ray-plane intersection test, giving us the distance to the ground
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        // If the ray does not intersect the ground
        // (the camera is not looking towards the ground), we can't do anything
        return;
    };

    // use the distance to compute the actual point on the ground in world-space
    let global_cursor = ray.get_point(distance);

    mycoords.global = global_cursor;
    eprintln!(
        "Global cursor coords: {}/{}/{}",
        global_cursor.x, global_cursor.y, global_cursor.z
    );
}
