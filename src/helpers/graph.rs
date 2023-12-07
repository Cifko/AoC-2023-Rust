use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<
    Vertex: Sized + PartialEq + Eq + Hash + Ord + Clone + Debug,
    Value: Ord + Hash + Clone + Debug,
> {
    pub edges: HashMap<Vertex, HashMap<Vertex, i64>>,
    pub vertices: HashMap<Vertex, Value>,
}

impl<
        Vertex: Sized + PartialEq + Eq + Hash + Ord + Clone + Debug,
        Value: Ord + Hash + Clone + Debug,
    > Graph<Vertex, Value>
{
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            vertices: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, index: Vertex, value: Value) {
        self.vertices.insert(index, value);
    }

    pub fn add_directed_edge(&mut self, u: Vertex, v: Vertex, weight: i64) {
        self.edges
            .entry(u)
            .or_insert(HashMap::new())
            .insert(v, weight);
    }

    pub fn add_edge(&mut self, u: Vertex, v: Vertex, weight: i64) {
        self.add_directed_edge(u.clone(), v.clone(), weight);
        self.add_directed_edge(v, u, weight);
    }

    fn update_backpack_with_value(
        &self,
        backpack: &BTreeSet<Value>,
        values: BTreeSet<Value>,
    ) -> BTreeSet<Value> {
        backpack.union(&values).cloned().collect()
    }

    pub fn mut_edges(&mut self, mutation: fn(i64) -> i64) {
        self.edges.iter_mut().for_each(|edge| {
            edge.1.iter_mut().for_each(|edge| {
                *edge.1 = mutation(*edge.1);
            })
        });
    }

    pub fn djikstra(
        &self,
        start: Vertex,
        moves: i64,
        backpack: BTreeSet<Value>,
        can_move_between: fn(&Value, &Value, &BTreeSet<Value>) -> bool,
        can_move_to: fn(&Value, &BTreeSet<Value>) -> bool,
        update_backpack: fn(&Value) -> BTreeSet<Value>,
    ) -> HashMap<Vertex, HashMap<BTreeSet<Value>, i64>> {
        let mut backlog = BinaryHeap::new();
        backlog.push(Reverse((moves, backpack, start)));
        let mut cached = HashMap::new();
        while !backlog.is_empty() {
            let (moves, backpack, position) = backlog.pop().unwrap().0;
            if cached
                .entry(position.clone())
                .or_insert(HashMap::new())
                .get(&backpack)
                .is_some_and(|m| *m <= moves)
            {
                continue;
            }
            cached
                .get_mut(&position)
                .unwrap()
                .insert(backpack.clone(), moves);
            for v in self.edges.get(&position).unwrap().keys() {
                if can_move_between(&self.vertices[&position], &self.vertices[v], &backpack)
                    && can_move_to(&self.vertices[v], &backpack)
                {
                    backlog.push(Reverse((
                        moves + self.edges[&position][v],
                        self.update_backpack_with_value(
                            &backpack,
                            update_backpack(&self.vertices[v]),
                        ),
                        v.clone(),
                    )));
                }
            }
        }
        cached
    }
}
