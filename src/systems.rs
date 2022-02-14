use crate::camera::*;
use bevy::prelude::*;
use matrixlab::matrix::sparse::SparseMatrix;
use rand::prelude::*;

// spawns the camera/lighting and birbs with random initial velocities
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    embedding: ResMut<Embedding>,
) {
    for coord in embedding.coords.iter() {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 1,
            })),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(coord.0, coord.1, coord.2),
            ..Default::default()
        });
    }
    commands.insert_resource(AmbientLight {
        brightness: 0.75,
        ..Default::default()
    });

    spawn_camera(commands);
}
/*
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut count: ResMut<Coords>,
) {
    if count.count < 50000 {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let rand_values: Vec<f32> = (0..6).map(|_| rng.gen_range(-1.0..1.0)).collect();
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.005,
                    subdivisions: 1,
                })),
                material: materials
                    .add(Color::rgb(rand_values[3], rand_values[4], rand_values[5]).into()),
                transform: Transform::from_xyz(rand_values[0], rand_values[1], rand_values[2]),
                ..Default::default()
            });
        }
        count.count += 1000;
        println!("{}", count.count);
    }
}
*/
