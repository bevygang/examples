use bevy::color::palettes::css::{GREEN, RED, YELLOW};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>) {

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Mesh2d(meshes.add(Rectangle::new(1000.0,100.0))))
        .insert(MeshMaterial2d(materials.add(Color::from(RED))))
        .insert(Transform::from_xyz(0.0, -400.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Mesh2d(meshes.add(Circle::new(50.0))))
        .insert(MeshMaterial2d(materials.add(Color::from(GREEN))))
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>,
                       mut commands: Commands,
                       mut meshes: ResMut<Assets<Mesh>>,
                       mut materials: ResMut<Assets<ColorMaterial>>) {
    for transform in positions.iter() {
        let height = transform.translation.y;
        if height > 0.0 && height < 17.0 {
            commands
                .spawn(RigidBody::Dynamic)
                .insert(Mesh2d(meshes.add(Circle::new(50.0))))
                .insert(MeshMaterial2d(materials.add(Color::from(YELLOW))))
                .insert(Collider::ball(50.0))
                .insert(Restitution::coefficient(0.7))
                .insert(Transform::from_xyz(20.0, 400.0, 0.0));
        }
    }
}