pub mod camera;
pub mod systems;

use crate::camera::*;
use crate::systems::*;
use bevy::prelude::*;
use matrixlab::matrix::sparse::SparseMatrix;
use matrixlab::MatrixElement;

pub struct Embedding {
    coords: Vec<(f32, f32, f32)>,
    partition: Vec<SparseMatrix<usize>>,
    matrix: SparseMatrix<usize>,
    labels: Vec<String>,
}

impl Embedding {
    pub fn new(
        coords: Vec<(f32, f32, f32)>,
        partition: Vec<SparseMatrix<usize>>,
        matrix: SparseMatrix<usize>,
        labels: Vec<String>,
    ) -> Self {
        Self {
            coords,
            partition,
            matrix,
            labels,
        }
    }
}

pub fn run(embedding: Embedding) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .insert_resource(embedding)
        .add_system(pan_orbit_camera.system())
        .run();
}

pub fn load_partfile() -> Vec<SparseMatrix<usize>> {
    let mut string = std::fs::read_to_string("temp/partfile.coo").unwrap();
    string = string.replace("(", " ");
    string = string.replace(")", " ");
    string = string.replace(",", " ");

    let mut partitions: Vec<SparseMatrix<usize>> = Vec::new();

    for matrix in string.trim().split("\n\n") {
        let mut elements = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for entry in matrix.trim().split("\n") {
            let element: Vec<usize> = entry
                .trim()
                .split_ascii_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            elements.push(MatrixElement::new(element[0], element[1], 1));
            if element[0] + 1 > rows {
                rows = element[0] + 1;
            }
            if element[1] + 1 > cols {
                cols = element[1] + 1;
            }
            println!("{} {}", element[0], element[1]);
        }
        partitions.push(SparseMatrix::new(rows, cols, elements).unwrap());
    }

    partitions
}

pub fn load_coords() -> Vec<(f32, f32, f32)> {
    let string = std::fs::read_to_string("temp/embedding.txt").unwrap();
    string
        .trim()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<f32>>()
        .chunks(3)
        .map(|coord| (coord[0], coord[1], coord[2]))
        .collect()
}
