use std::collections::LinkedList;
use crate::topological_order;
use crate::dijkstra;

pub type VertexId = usize;

pub trait Graph {
    fn is_cyclic(&mut self) -> bool;
    fn path(&self, from: VertexId, to: VertexId) -> bool;
}
#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    White,
    Gray,
    Black,
}

#[derive(Debug)]
pub enum GraphError {
    CycleFoundInGraph,
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
    pub weight: isize,
}

impl Edge {
    pub fn new(to: VertexId, weight: isize) -> Self {
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

    pub fn topo_sort_from(&self, from: VertexId) -> Result<Vec<VertexId>, GraphError> {
        let mut colors = vec![Color::White; self.graph.len()];
        let mut topo: Vec<VertexId> = Vec::new();

        match topo_sort(&self, &mut colors, &mut topo, from) {
            Ok(()) => {
                topo.reverse();
                Ok(topo)
            },
            Err(GraphError::CycleFoundInGraph) => Err(GraphError::CycleFoundInGraph),
        }
    }
}

impl Default for WGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph for WGraph {
    fn is_cyclic(&mut self) -> bool {
        let mut colors = vec![Color::White; self.graph.len()];

        for v_idx in 0..self.graph.len() {
            if colors[v_idx] == Color::White && cycle_dfs(self, &mut colors, v_idx) {
                return true;
            };
        }
        false
    }
    fn path(&self, from: VertexId, to: VertexId) -> bool {
        match topological_order::path(from, to, self) {
            Ok(val) => val,
            Err(GraphError::CycleFoundInGraph) => dijkstra::path(from, to, self)
        }
    }
}

fn cycle_dfs(graph: &WGraph, colors: &mut [Color], v: usize) -> bool {
    colors[v] = Color::Gray;

    for next in &graph.graph[v].out_edges {
        match colors[next.to] {
            Color::White => {
                cycle_dfs(graph, colors, next.to);
            }
            Color::Gray => return true,
            Color::Black => (),
        }
    }

    colors[v] = Color::Black;
    false
}

fn topo_sort(graph: &WGraph, colors: &mut [Color], order: &mut Vec<VertexId>, v: VertexId) -> Result<(), GraphError> {
    colors[v] = Color::Gray;

    for next in &graph.graph[v].out_edges {
        match colors[next.to] {
            Color::White => match topo_sort(graph, colors, order, next.to) {
                Err(e) => return Err(e),
                _ => ()
            },
            Color::Gray => return Err(GraphError::CycleFoundInGraph),
            Color::Black => (),
        }
    }

    colors[v] = Color::Black;
    order.push(v);

    Ok(())
}