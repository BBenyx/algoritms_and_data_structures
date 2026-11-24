use crate::graph::{GraphError, VertexId, WGraph};

pub fn path(from: VertexId, to: VertexId, graph: &WGraph) -> Result<bool, GraphError> {
    let mut weigths = vec![isize::MAX; graph.graph.len()];
    weigths[from] = 0;

    for i in 0..graph.graph.len() {

        let mut relaxed = false;
        for v_idx in 0..graph.graph.len() {
            if weigths[v_idx] == isize::MAX {
                continue
            }

            for edge in &graph.graph[v_idx].out_edges {
                if weigths[edge.to] > weigths[v_idx] + edge.weight {
                    relaxed = true;
                    weigths[edge.to] = weigths[v_idx] + edge.weight;
                }
            }
        }
        if !relaxed {
            break
        }
        if relaxed && i == graph.graph.len() - 1 {
            // We have a negative weighted cycle
            return Err(GraphError::NegativeWeightedGraphFoundInGraph)
        }
    }
    if weigths[to] != isize::MAX {
        Ok(true)
    } else {
        Ok(false)
    }
}