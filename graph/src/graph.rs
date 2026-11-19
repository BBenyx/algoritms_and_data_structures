use std::collections::LinkedList;

type VertexId = usize;

pub trait Graph {
    fn cyclic(&mut self) -> bool;
}
#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    White,
    Gray,
    Black,
}
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Vertex {
    pub data: usize,
    pub out_edges: LinkedList<Edge>,
}

impl Vertex {
    pub fn new(data: usize) -> Self {
        Self {
            data,
            out_edges: LinkedList::new(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Edge {
    pub to: VertexId,
    pub weight: usize,
}

impl Edge {
    pub fn new(to: VertexId, weight: usize) -> Self {
        Self { to, weight }
    }
}

#[derive(Debug)]
pub struct WGraph {
    pub graph: Vec<Vertex>,
}

impl WGraph {
    pub fn new() -> WGraph {
        Self { graph: Vec::new() }
    }

    fn cycle_dfs(colors: &mut [Color], graph: &WGraph, v: usize) -> bool {
        colors[v] = Color::Gray;

        for next in &graph.graph[v].out_edges {
            match colors[next.to] {
                Color::White => {
                    Self::cycle_dfs(colors, graph, next.to);
                }
                Color::Gray => return true,
                Color::Black => (),
            }
        }

        colors[v] = Color::Black;
        false
    }
}

impl Default for WGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph for WGraph {
    fn cyclic(&mut self) -> bool {
        let mut colors = vec![Color::White; self.graph.len()];

        for v_idx in 0..self.graph.len() {
            if colors[v_idx] == Color::White && Self::cycle_dfs(&mut colors, self, v_idx) {
                return true;
            };
        }

        false
    }
}
