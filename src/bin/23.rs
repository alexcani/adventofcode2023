use advent_of_code_2023::read_lines_as_vec;
use advent_of_code_2023::util::grid::*;
use advent_of_code_2023::util::point::*;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec;

// Huge credits to https://github.com/maneatingape for providing inspiration for this problems solution on
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day23.rs
// And also inspiration for the grid and point implementations

struct Puzzle {
    start_idx: usize,
    end_idx: usize,
    edges: Vec<u64>,
    undirected_edges: Vec<u64>,
    weights: Vec<Vec<u32>>,
    extra: u32,
}

fn parse(input: &[String]) -> Puzzle {
    let mut grid = Grid::parse(input);
    let width = grid.width as i64;
    let height = grid.height as i64;

    // To make things easier we'll replace start and end points with # and add the cost later
    // This allows us to not check for any boundaries
    grid[Point::new(1, 0)] = b'#';
    grid[Point::new(width - 2, height - 1)] = b'#';

    // New start and end points
    let start = Point::new(1, 1);
    let end = Point::new(width - 2, height - 2);

    // Replace start and end with P for 'point of interest'
    grid[start] = b'P';
    grid[end] = b'P';

    // Map holds the point of interest along with a unique index
    let mut poi = HashMap::new();
    poi.insert(start, 0);
    poi.insert(end, 1);

    // Find all other points of interest (junctions)
    for y in 1..height - 1 {
        // 0th row and 0th column are walls, as well as the last row and column
        for x in 1..width - 1 {
            let pos = Point::new(x, y);
            if grid[pos] != b'#' {
                let neighbors = ORTHOGONALS.iter().map(|&dir| pos + dir)
                    .filter(|e| grid[*e] != b'#')
                    .collect::<Vec<_>>();
                if neighbors.len() > 2 {
                    grid[pos] = b'P';
                    poi.insert(pos, poi.len());
                }
            }
        }
    }

    let n_pois = poi.len();

    // Graph with the directed edges. Value is a bitmask where each bit represents another POI
    let mut directed_edges: Vec<u64> = vec![0; n_pois];
    let mut undirected_edges: Vec<u64> = vec![0; n_pois];

    // Weights of the edges
    let mut weights = vec![vec![0; n_pois]; n_pois];

    let mut queue = VecDeque::new();
    // Find the distance between each point of interest using BFS
    for (&start, &start_index) in &poi {
        queue.push_back((start, 0, true));  // (pos, cost, forward)
        grid[start] = b'#';  // Mark as visited/forest

        while let Some((point, cost, forward)) = queue.pop_front() {
            for dir in ORTHOGONALS {  // get all neighbors
                let next = point + dir;
                match grid[next] {
                    b'#' => (),
                    b'P' => {  // found a POI from `start`
                        let poi_index = poi[&next];
                        if forward {
                            directed_edges[start_index] |= 1 << poi_index;
                        } else {
                            directed_edges[poi_index] |= 1 << start_index;
                        }

                        undirected_edges[start_index] |= 1 << poi_index;
                        undirected_edges[poi_index] |= 1 << start_index;

                        weights[start_index][poi_index] = cost + 1;
                        weights[poi_index][start_index] = cost + 1;
                    },
                    b'.' => {  // keep walking
                        queue.push_back((next, cost + 1, forward));
                        grid[next] = b'#';
                    },
                    _ => {  // slope ^, >, <, v
                        let slope_dir = Point::from(grid[next]);
                        let same = dir == slope_dir;  // neighbour points forward
                        queue.push_back((next, cost+1, same && forward));  // once direction inverts it can never go forward again
                        grid[next] = b'#';
                    }
                }
            }
        }
    }

    // Heuristics to make the algorithm faster
    // Since start and end nodes only connect to one other node, we can remove them from the graph by compressing the graph
    let start = undirected_edges[0].trailing_zeros() as usize;
    let end = undirected_edges[1].trailing_zeros() as usize;  // indexes of edges to which start and end connect

    // The nodes in the graph actually form a grid where all nodes have 4 neighbors, except the ones in the perimeter, which ofc have 3
    // Nodes on the borders imply you can only go down or right, otherwise a dead end will inevitably happen.
    // By removing the dead edge on our "undirected" graph possible dead ends are pruned thus reducing the search space

    let mut nodes_in_perimeter = 0;
    for (i, edges) in undirected_edges.iter().enumerate() {
        if edges.count_ones() < 4 {  // not fully connected
            nodes_in_perimeter |= 1 << i;
        }
    }

    for (i, edges) in undirected_edges.iter_mut().enumerate() {
        if edges.count_ones() < 4 {
            // Final edges for this node will be the directed edges for this node (outwards) and the undirected edges for the neighbors not in perimeter (inwards)
            *edges = (*edges & !nodes_in_perimeter) | directed_edges[i];
        }
    }

    let extra = 2 + weights[0][start] + weights[1][end];  // compensate for start and end points being moved and the graph compressed

    Puzzle {
        start_idx: start,
        end_idx: end,
        edges: directed_edges,
        undirected_edges,
        weights,
        extra,
    }

}

fn part1(input: &Puzzle) -> u32 {
    // Holds the largest cost to reach the given node in the graph
    let mut cost = vec![0; input.edges.len()];

    let mut queue = VecDeque::new();
    queue.push_back(input.start_idx);

    while let Some(node) = queue.pop_front() {
        let mut neighbors = input.edges[node];
        while neighbors > 0 {
            // Index of the next node
            let next = neighbors.trailing_zeros() as usize;
            neighbors ^= 1 << next;  // remove the bit

            // The cost is either what it costs now or the cost through this path, whichever is larger
            cost[next] = cost[next].max(cost[node] + input.weights[node][next]);
            queue.push_back(next);
        }
    }

    // The cost to reach the end node is the answer, added the compensation for the optimizations
    cost[input.end_idx] + input.extra
}

fn dfs(input: &Puzzle, start: usize, seen_nodes: u64, cost: u32) -> u32 {
    if start == input.end_idx {
        return cost;
    }

    // Neighbors without the ones we've already seen
    let mut neighbors = input.undirected_edges[start] & !seen_nodes;
    let mut result = 0;
    while neighbors > 0 {
        let next = neighbors.trailing_zeros() as usize;
        neighbors ^= 1 << next;

        result = result.max(dfs(input, next, seen_nodes | 1 << next, cost + input.weights[start][next]));
    }

    result
}

fn part2(input: &Puzzle) -> u32 {
    dfs(input, input.start_idx, 1 << input.start_idx, 0) + input.extra
}

fn main() {
    let _lines = read_lines_as_vec("inputs/23.txt").unwrap();
    let _example = r#"#.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#"#
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();

    let now = std::time::Instant::now();
    let map = parse(&_lines);
    println!("Parsing took {}us", now.elapsed().as_micros());

    let now = std::time::Instant::now();
    let r1 = part1(&map);
    println!("Part 1 took {}us", now.elapsed().as_micros());
    println!("Part 1: {}", r1);

    let now = std::time::Instant::now();
    let r2 = part2(&map);
    println!("Part 2 took {}ms", now.elapsed().as_millis());
    println!("Part 2: {}", r2);
}
