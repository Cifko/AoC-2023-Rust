use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Graph<'a, Vertex: ?Sized + PartialEq + Eq + Hash, Value> {
    pub edges: HashMap<&'a Vertex, HashMap<&'a Vertex, f64>>,
    pub vertices: HashMap<&'a Vertex, Option<Value>>,
}

impl<'a, Vertex: ?Sized + PartialEq + Eq + Hash, Value> Graph<'a, Vertex, Value> {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            vertices: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, u: &'a Vertex, value: Option<Value>) {
        self.vertices.insert(u, value);
    }

    pub fn add_directed_edge(&mut self, u: &'a Vertex, v: &'a Vertex, weight: f64) {
        self.edges
            .entry(u)
            .or_insert(HashMap::new())
            .insert(v, weight);
    }

    pub fn add_edge(&mut self, u: &'a Vertex, v: &'a Vertex, weight: f64) {
        self.add_directed_edge(u, v, weight);
        self.add_directed_edge(v, u, weight);
    }
}
