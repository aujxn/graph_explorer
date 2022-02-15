pub mod camera;
pub mod systems;

use crate::camera::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy_mod_picking::*;
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
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup.system())
        .insert_resource(embedding)
        .add_system(pan_orbit_camera.system())
        .run();
}

pub fn load_coords() -> (Vec<(f32, f32, f32)>, Vec<SparseMatrix<usize>>) {
    let data: Vec<String> = std::fs::read_to_string("../recipe_analysis/temp/embedding.txt")
        .unwrap()
        .trim()
        .split("\n\n\n")
        .map(|x| x.to_string())
        .collect();

    let coords = data[0]
        .trim()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<f32>>()
        .chunks(3)
        .map(|coord| (coord[0], coord[1], coord[2]))
        .collect();

    println!("{:?}", data);
    let hierarchy = data[1..]
        .iter()
        .map(|matrix| {
            let mut matrix = matrix.trim().split("\n");
            let mut dimension = matrix.next().unwrap().split_whitespace();
            let rows = dimension.next().unwrap().parse().unwrap();
            let cols = dimension.next().unwrap().parse().unwrap();
            let data = matrix
                .map(|triplet| {
                    let triplet: Vec<usize> = triplet
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();
                    MatrixElement(triplet[0], triplet[1], triplet[2])
                })
                .collect();
            SparseMatrix::new(rows, cols, data).unwrap()
        })
        .collect();

    (coords, hierarchy)
}

pub fn load_labels() -> Vec<String> {
    let string = std::fs::read_to_string("../recipe_analysis/temp/ingredient_labels.txt").unwrap();
    string.split("\n").map(|label| label.into()).collect()
}

pub fn load_matrix() -> SparseMatrix<usize> {
    let string =
        std::fs::read_to_string("../recipe_analysis/temp/ingredient_ingredient.coo").unwrap();
    let mut elements = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for element in string
        .trim()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(3)
    {
        elements.push(MatrixElement::new(element[0], element[1], element[2]));
        if element[0] + 1 > rows {
            rows = element[0] + 1;
        }
        if element[1] + 1 > cols {
            cols = element[1] + 1;
        }
    }
    SparseMatrix::new(rows, cols, elements).unwrap()
}
