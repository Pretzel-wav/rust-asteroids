use bevy::{prelude::*, render::mesh::PrimitiveTopology, render::mesh::Indices, sprite::MaterialMesh2dBundle};

const VIEWPORT_WIDTH: usize = 1280;
const VIEWPORT_HEIGHT: usize = 720;
const VIEWPORT_MAX_X: f32 = VIEWPORT_WIDTH as f32 / 2.0;
const VIEWPORT_MIN_X: f32 = -VIEWPORT_MAX_X;
const VIEWPORT_MAX_Y: f32 = VIEWPORT_HEIGHT as f32 / 2.0;
const VIEWPORT_MIN_Y: f32 = -VIEWPORT_MAX_Y;
const ASTEROID_VELOCITY: f32 = 1.0;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_position)
        .add_system(sync_translate_transform.after(update_position))
        .add_system(sync_asteroid_scale_transform)
        .run();

}

enum AsteroidSize {
    Big,
    Medium,
    Small,
}

#[derive(Component)]
struct Starship;

#[derive(Component)]
struct Asteroid {
    size: AsteroidSize,
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

fn create_starship_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.5, 0.0], [-0.25, -0.5, 0.0], [0.25, -0.5, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0,1,2])));
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.5, 0.0], [0.0, 1.0], [1.0, 1.0]],
    );
    mesh
}

fn get_random_point() -> Vec2 {
    Vec2::new(
        (rand::random::<f32>() * 2.0 - 1.0) * (VIEWPORT_WIDTH as f32) / 2.0,
        (rand::random::<f32>() * 2.0 - 1.0) * (VIEWPORT_HEIGHT as f32) / 2.0,
    )
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(1280.0, 720.0);
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn()
        .insert(Starship)
        .insert(Position(Vec2::splat(0.0)))
        .insert(Velocity(Vec2::splat(0.0)))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(create_starship_mesh())
                .into(),
            transform: Transform::default().with_scale(Vec3::splat(50.)),
            material: materials
                .add(ColorMaterial::from(Color::rgba(30., 0., 120., 1.0))),
            ..default()
    });

    for _ in 0..6{
        commands.spawn()
            .insert(Asteroid {
                size: AsteroidSize::Big,
            })
            .insert(Position(get_random_point()))
            .insert(Velocity(get_random_point().normalize() * ASTEROID_VELOCITY))
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle::new(0.5)))
                    .into(),
                transform: Transform::default()
                    .with_translation(Vec3::new(0.0, 0.0, 1.0)),
                material: materials
                    .add(ColorMaterial::from(Color::rgba(0., 30., 0., 1.0))),
                ..default()
            });
    }
}

fn sync_translate_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation = 
            Vec3::new(position.0.x, position.0.y, transform.translation.z);
    }
}

fn sync_asteroid_scale_transform(mut query: Query<(&Asteroid, &mut Transform)>) {
    for (asteroid, mut transform) in &mut query {
        transform.scale = Vec3::splat( match asteroid.size {
            AsteroidSize::Big => 100.0,
            AsteroidSize::Medium => 80.0,
            AsteroidSize::Small => 60.0,
        })
    }
}

fn update_position(mut query: Query<(&Velocity, &Transform, &mut Position)>) {
    for (velocity, transform, mut position) in &mut query {

        let mut new_position = position.0 + velocity.0;
        let half_scale = transform.scale.max_element() / 2.0;

        if new_position.x > VIEWPORT_MAX_X + half_scale {
            new_position.x = VIEWPORT_MIN_X - half_scale;
        } else if new_position.x < VIEWPORT_MIN_X - half_scale {
            new_position.x += VIEWPORT_MAX_X + half_scale;
        }

        if new_position.y > VIEWPORT_MAX_Y  + half_scale {
            new_position.y = VIEWPORT_MIN_Y - half_scale;
        } else if new_position.y < VIEWPORT_MIN_Y  - half_scale {
            new_position.y = VIEWPORT_MAX_Y + half_scale;
        }

        position.0 = new_position;
    }
}
