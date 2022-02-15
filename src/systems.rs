use crate::camera::*;
use crate::Embedding;
use bevy::prelude::*;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle,
    PickingCameraBundle,
};
use matrixlab::matrix::sparse::SparseMatrix;
use rand::prelude::*;

// spawns the camera/lighting and birbs with random initial velocities
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    embedding: ResMut<Embedding>,
) {
    let last = embedding.partition[1]
        .safe_sparse_mat_mul(&embedding.partition[0])
        .unwrap();

    /*
    for (i, part) in embedding.partition.iter().skip(1).enumerate() {
        let next = part.safe_sparse_mat_mul(&groups[i]).unwrap();
        groups.push(next);
    }
    let _last = groups.pop().unwrap();
    let last = groups.pop().unwrap();
    */

    let colors = [
        Color::FUCHSIA,
        Color::GREEN,
        Color::RED,
        Color::CYAN,
        Color::BLUE,
        Color::OLIVE,
        Color::PURPLE,
        Color::PINK,
    ];

    let color_names = [
        "fuchsia", "green", "red", "cyan", "blue", "olive", "purple", "pink",
    ];

    let coarse = last
        .safe_sparse_mat_mul(&embedding.matrix)
        .unwrap()
        .safe_sparse_mat_mul(&last.transpose())
        .unwrap();

    let mut quality = Vec::new();
    for (i, (cols, weights)) in coarse.row_iter().enumerate() {
        let mut self_connections = 0;
        let mut other_connections = 0;
        for (j, weight) in cols.iter().zip(weights.iter()) {
            if i == *j {
                self_connections = *weight;
            } else {
                other_connections += *weight;
            }
        }
        quality.push((self_connections, other_connections));
    }

    let mut assigned_colors = vec![0; last.num_columns()];
    for (i, (cols, _vals)) in last.row_iter().enumerate() {
        println!(
            "\ngroup {}, {} edges in: {} edges out: {} ratio: {}",
            i,
            color_names[i],
            quality[i].0,
            quality[i].1,
            quality[i].0 as f32 / quality[i].1 as f32
        );
        for j in cols {
            print!("{}, ", embedding.labels[*j]);
            assigned_colors[*j] = i;
        }
        println!();
    }

    //println!("{:?}", assigned_colors);

    for (i, coord) in embedding.coords.iter().enumerate() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.3,
                    subdivisions: 1,
                })),
                material: materials.add(colors[assigned_colors[i]].into()),
                transform: Transform::from_xyz(coord.0, coord.1, coord.2),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default());
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
