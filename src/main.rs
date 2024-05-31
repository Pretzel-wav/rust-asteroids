use bevy::{prelude::*, render::mesh::PrimitiveTopology, render::mesh::Indices, sprite::MaterialMesh2dBundle};

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();

}

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes
            .add(create_starship_mesh())
            .into(),
        transform: Transform::default().with_scale(Vec3::splat(50.)),
        material: materials
            .add(ColorMaterial::from(Color::rgba(30., 0., 120., 1.0))),
        ..default()
    });

    for _ in 0..10{
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Circle::new(0.5)))
                .into(),
            transform: Transform::default().with_scale(Vec3::splat(100.))
                .with_translation(Vec3::new(
                    (rand::random::<f32>() * 2.0 - 1.0) * 600.0,
                    (rand::random::<f32>() * 2.0 - 1.0) * 600.0,
                    0.0,
                )),
            material: materials
                .add(ColorMaterial::from(Color::rgba(0., 30., 0., 1.0))),
            ..default()
        });
    }
}
