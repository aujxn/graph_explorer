fn main() {
    let (coords, partition) = graph_explorer::load_coords();
    let matrix = graph_explorer::load_matrix();
    let ingredient_list = graph_explorer::load_labels();

    let embedding = graph_explorer::Embedding::new(coords, partition, matrix, ingredient_list);

    graph_explorer::run(embedding);
}
