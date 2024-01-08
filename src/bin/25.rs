use advent_of_code_2023::read_lines_as_vec;
use std::collections::HashMap;
use nalgebra::DMatrix;

struct Input {
    adjacency_matrix: DMatrix<f32>,
    degree_matrix: DMatrix<f32>,
}

fn parse(input: &[String]) -> Input {
    // Since we don't know the number of nodes beforehand, let's use a map to store the node along with an index
    let mut nodes = HashMap::new();

    for line in input {
        let mut parts = line.trim().split(':');
        let from = parts.next().unwrap();
        let to = parts
            .next().unwrap().trim()
            .split(' ')
            .map(|s| s.trim()).collect::<Vec<_>>();

        // Make sure all nodes exist in the map
        let to_indices = to
            .iter()
            .map(|&s| {
                let i = nodes.len();
                nodes.entry(s).or_insert((vec![], i)).1
            })
            .collect::<Vec<_>>();

        let i = nodes.len();
        let from_node = nodes.entry(from).or_insert((vec![], i));
        from_node.0.extend(to_indices);  // add edges
        let from_index = from_node.1;

        // add reverse edges
        for node in to {
            nodes.get_mut(node).unwrap().0.push(from_index);
        }
    }

    // Now we have a map of nodes and their indices, let's build the adjacency matrix and the degree matrix
    let mut adj_matrix = DMatrix::<f32>::zeros(nodes.len(), nodes.len());
    let mut deg_matrix = DMatrix::<f32>::zeros(nodes.len(), nodes.len());
    for (edges, index) in nodes.values() {
        deg_matrix[(*index, *index)] = edges.len() as f32;
        for &edge in edges {
            adj_matrix[(*index, edge)] = 1.0;
        }
    }

    Input {
        adjacency_matrix: adj_matrix,
        degree_matrix: deg_matrix,
    }
}

fn part_1(input: &Input) -> u32 {
    // Perform spectral bisection
    let laplacian_matrix = &input.degree_matrix - &input.adjacency_matrix;

    let eigen_decomp = laplacian_matrix.symmetric_eigen();  // This needs to run in release mode
    let eigen_values = eigen_decomp.eigenvalues;
    let eigen_vectors = eigen_decomp.eigenvectors;

    // Get index of second smallest eigenvalue
    let mut min_index = 0;
    let mut min_value = f32::MAX;
    let mut second_min_index = 0;
    let mut second_min_value = f32::MAX;
    for (i, &value) in eigen_values.iter().enumerate() {
        if value < min_value {
            second_min_index = min_index;
            second_min_value = min_value;
            min_index = i;
            min_value = value;
        } else if value < second_min_value {
            second_min_index = i;
            second_min_value = value;
        }
    }

    // Get the corresponding eigenvector
    let second_smallest_eigenvector = eigen_vectors.column(second_min_index).clone_owned();
    let mut positive = 0;
    let mut negative = 0;
    second_smallest_eigenvector.iter().for_each(|&x| {
        if x >= 0.0 {
            positive += 1;
        } else {
            negative += 1;
        }
    });
    println!("Positive: {}, Negative: {}", positive, negative);
    positive * negative
}

fn main() {
    let _lines = read_lines_as_vec("inputs/25.txt").unwrap();
    let _example = r#"jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr"#
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<_>>();

    let now = std::time::Instant::now();
    let input = parse(&_lines);
    println!("Parsing took {}ms", now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1: {} ({}ms)", result, now.elapsed().as_millis());
}
