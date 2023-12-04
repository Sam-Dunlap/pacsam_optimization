use graph_builder::prelude::*;
use std::{error::Error, fs};

struct Edge {
    vertices: (usize, usize),
    length: usize,
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Edge {
            vertices: (self.vertices.0, self.vertices.1),
            length: self.length,
        }
    }
}

// expecting most of the options in these functions because we know from the data input that they will
// always return Some(_), so more error handling is unnecessary.
pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let graph = build_graph(contents);
    fix_culdesacs(&graph);
    eulerize(&graph);
    // let path = find_cycle(&graph);
    // println!("{}", alphabetize(&path));
    // println!("{} miles", length_miles(&path, &graph));
    Ok(())
}

fn alphabetize(path: &Vec<usize>) -> String {
    // nodes are numeric but the graph I create in Google earth uses letters for the nodes. this converts back
    // for easier readability
    let alphabet = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let mut alpha_path: String = "".to_string();
    for (i, node) in path.iter().enumerate() {
        alpha_path.push_str(alphabet[*node]);
        if i < path.len() - 1 {
            alpha_path.push_str(" -- ");
        }
    }
    alpha_path
}

fn length_miles(path: &Vec<usize>, graph: &UndirectedALGraph<usize, (), usize>) -> f64 {
    // the weights of each edge are expressed as feet. this finds each edge along the final path and sums them,
    // then returns the value expressed in miles (truncated to two decimal places)
    let mut ft = 0.0;
    let mut count = 0;
    while count < path.len() - 1 {
        let v1 = path[count];
        let v2 = path[count + 1];
        let edge = graph
            .neighbors_with_values(v1)
            .find(|edge| edge.target == v2)
            .expect("this exists");
        ft += edge.value as f64;
        count += 1;
    }
    let miles = ft / 5280.0;
    let miles_two_decimals = f64::trunc(miles * 100.0) / 100.0;
    miles_two_decimals
}

fn fix_culdesacs(graph: &UndirectedALGraph<usize, (), usize>) {
    // each node with degree 1 is a cul de sac / dead end, and the only way to include a cul de sac on an euler cycle is to
    // go into it, then come back out. this function adds those returning edges to each cul de sac before running the rest
    // of the algorithm.
    let mut nodes_with_degree_one: Vec<usize> = vec![];
    for i in 0..graph.node_count() {
        if graph.degree(i) == 1 {
            nodes_with_degree_one.push(i);
        }
    }
    if nodes_with_degree_one.len() > 0 {
        for node in nodes_with_degree_one {
            let neighbor = graph
                .neighbors_with_values(node)
                .next()
                .expect("there will always be a neighbor");
            let _ = graph.add_edge_with_value(node, neighbor.target, neighbor.value);
        }
    }
}

fn eulerize(graph: &UndirectedALGraph<usize, (), usize>) {
    // the neighborhoods will not usually have an euler cycle immediately.
    // we use the following method to create one by duplicating edges until there are no odd-degree nodes
    let mut nodes_with_odd_degree: Vec<usize> = vec![];
    for i in 0..graph.node_count() {
        if graph.degree(i) % 2 != 0 {
            nodes_with_odd_degree.push(i);
        }
    }
    if nodes_with_odd_degree.len() == 0 {
        return;
    }
    // construct a complete graph where the nodes are the set of odd degree nodes from the original, and their
    // connected edges are the shortest path between them
    let mut shortest_path_trees: Vec<Vec<Vertex>> = vec![];
    for vertex in &nodes_with_odd_degree {
        shortest_path_trees.push(dijkstra(graph, *vertex));
    }
    let mut new_edges: Vec<(usize, usize, usize)> = vec![];
    let mut trimmed_sp_trees: Vec<Vec<Vertex>> = vec![];
    for tree in shortest_path_trees {
        let mut tree_filter = tree
            .iter()
            .filter(|&vertex| nodes_with_odd_degree.contains(&vertex.idx));
        let mut trimmed_tree = vec![];
        while let Some(vertex) = tree_filter.next() {
            trimmed_tree.push(Vertex::new(vertex.idx, vertex.distance_from_u));
        }
        trimmed_sp_trees.push(trimmed_tree);
    }
    for (new_i, tree) in trimmed_sp_trees.iter().enumerate() {
        for vertex in tree {
            if nodes_with_odd_degree[new_i] == vertex.idx {
                continue;
            }
            let new_j = nodes_with_odd_degree
                .iter()
                .position(|node| *node == vertex.idx)
                .expect("this should exist");
            if !new_edges.contains(&(new_j, new_i, vertex.distance_from_u)) {
                new_edges.push((new_i, new_j, vertex.distance_from_u));
            }
        }
    }
    let graph2: UndirectedALGraph<usize, (), usize> =
        GraphBuilder::new().edges_with_values(new_edges).build();
}

struct Vertex {
    idx: usize,
    distance_from_u: usize,
}
impl Vertex {
    fn new(idx: usize, distance_from_u: usize) -> Self {
        Vertex {
            idx,
            distance_from_u,
        }
    }
    fn set_distance(&mut self, distance: usize) {
        self.distance_from_u = distance;
    }
}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance_from_u.cmp(&other.distance_from_u)
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance_from_u.cmp(&other.distance_from_u))
    }
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        (self.idx, self.distance_from_u) == (other.idx, other.distance_from_u)
    }
}
impl Eq for Vertex {}

fn dijkstra(graph: &UndirectedALGraph<usize, (), usize>, initial: usize) -> Vec<Vertex> {
    let mut unvisited: Vec<Vertex> = vec![];
    let mut sp_tree: Vec<Vertex> = vec![];
    for i in 0..graph.node_count() {
        unvisited.push(Vertex::new(i, usize::MAX));
    }
    let u: &mut Vertex = unvisited.iter_mut().find(|v| v.idx == initial).expect("ok");
    u.set_distance(0);
    let mut current = initial;
    while !unvisited.is_empty() {
        let cumulative_dist = unvisited
            .iter()
            .find(|u| u.idx == current)
            .expect("ok")
            .distance_from_u;
        for neighbor in graph.neighbors_with_values(current) {
            if let Some(v) = unvisited.iter_mut().find(|u| u.idx == neighbor.target) {
                if neighbor.value + cumulative_dist < v.distance_from_u {
                    v.set_distance(neighbor.value + cumulative_dist);
                }
            }
        }
        let rm_idx = unvisited.iter().position(|u| u.idx == current).expect("ok");
        sp_tree.push(unvisited.swap_remove(rm_idx));
        if let Some(new_vertex) = unvisited.iter().min() {
            current = new_vertex.idx;
        }
    }
    sp_tree
}

fn find_cycle(graph: &UndirectedALGraph<usize, (), usize>) -> Vec<usize> {
    // get a vec of all edges, represented once each
    let mut edges: Vec<(usize, usize)> = vec![];
    for i in 0..graph.node_count() {
        for neighbor in graph.neighbors_with_values(i as usize) {
            if !edges.contains(&(neighbor.target as usize, i as usize)) {
                edges.push((i as usize, neighbor.target as usize));
            }
        }
    }
    // hierholzer's algorithm finds the euler circuit
    let mut path: Vec<usize> = vec![];
    let mut vertices_with_unused_edges: Vec<usize> = vec![0];
    while !vertices_with_unused_edges.is_empty() {
        let v1 = vertices_with_unused_edges[0];
        let neighbors: Vec<&(usize, usize)> = edges
            .iter()
            .filter(|edge| edge.0 == v1 || edge.1 == v1)
            .collect();
        if neighbors.len() == 0 {
            path.push(vertices_with_unused_edges.remove(0));
        } else {
            let chosen_edge = *neighbors[0];
            let rm_idx = edges
                .iter()
                .position(|edge| *edge == chosen_edge)
                .expect("exists");
            // edges is unordered, so swap_remove is faster with no downside
            edges.swap_remove(rm_idx);
            if chosen_edge.0 == v1 {
                vertices_with_unused_edges.insert(0, chosen_edge.1);
            } else {
                vertices_with_unused_edges.insert(0, chosen_edge.0);
            }
        }
    }
    path
}

fn build_graph(input: String) -> UndirectedALGraph<usize, (), usize> {
    // parse the input file
    let mut edges: Vec<(usize, usize, usize)> = vec![];
    let mut line_counter: usize = 0;
    for line in input.lines() {
        let edges_from_input: Vec<&str> = line.split(",").collect();
        for edge in edges_from_input {
            let vertex_and_weight: Vec<&str> = edge.split(":").collect();
            if vertex_and_weight.len() == 1 {
                continue;
            }
            let vertex = vertex_and_weight[0].parse::<usize>().unwrap();
            let weight = vertex_and_weight[1].parse::<usize>().unwrap();
            edges.push((line_counter, vertex, weight));
        }
        line_counter += 1;
    }
    let graph: UndirectedALGraph<usize, (), usize> =
        GraphBuilder::new().edges_with_values(edges).build();

    graph
}
