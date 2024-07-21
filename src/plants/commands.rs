use crate::ui::elements::AquaElementsState;
use crate::GroundCoords;
use bevy::{input::common_conditions::input_pressed, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlantsCommandsPlugin;

impl Plugin for PlantsCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_sand
                .run_if(input_pressed(MouseButton::Left))
                .run_if(in_state(AquaElementsState::Plant)),
        );
    }
}

fn spawn_sand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    coords: ResMut<GroundCoords>,
) {
    /* Create the bouncing ball. */
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(coords.global.x, coords.global.y, coords.global.z),
            ..default()
        },
        Collider::ball(0.5),
        RigidBody::Dynamic,
        Restitution::coefficient(0.7),
        PickableBundle::default(),
    ));
}
