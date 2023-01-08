use std::rc::Rc;
use crate::graph::{Edge, Graph, Vertex};

/// `StepFn` is a function that takes a traversal and returns
/// a new traversal.
type StepFn<'a> = Box<dyn Fn(Vec<Rc<Vertex>>, &'a Graph) -> Vec<Rc<Vertex>> + 'a>;

pub struct Traversal<'a> {
    graph: &'a Graph,
    steps: Vec<StepFn<'a>>
}

impl<'a> Traversal<'a> {
    pub fn new(graph: &'a Graph, steps: Vec<StepFn<'a>>) -> Self {
        Self {
            graph,
            steps
        }
    }

    pub fn execute(&self) -> Vec<Rc<Vertex>> {
        let mut traversal = vec![
            self.graph.vertices
                .first()
                .expect("Graph must contain at least one vertex to execute a traversal.")
                .clone()
        ];

        for step in &self.steps {
            traversal = step(traversal.clone(), self.graph);
        }

        traversal
    }

    pub fn has(mut self, key: &'a str, value: &'a str) -> Self {
        self.steps.push(has(key, value));
        self
    }

    pub fn out(mut self) -> Self {
        self.steps.push(expand());
        self
    }

    pub fn r#in(mut self) -> Self {
        self.steps.push(r#in());
        self
    }

    pub fn in_e(&self) -> Vec<Edge> {
        self.graph.edges
            .clone()
            .into_iter()
            .filter(|e| self.execute().contains(&e.destination))
            .collect()
    }

    pub fn out_e(&self) -> Vec<Edge> {
        self.graph.edges
            .clone()
            .into_iter()
            .filter(|e| self.execute().contains(&e.source))
            .collect()
    }
}

pub fn find_one<'a>(vertex: Rc<Vertex>) -> StepFn<'a> {
    Box::new(move |_, graph| {
        graph.vertices
            .clone()
            .into_iter()
            .filter(|v| v.id == vertex.id)
            .collect()
    })
}

pub fn expand<'a>() -> StepFn<'a> {
    Box::new(move |traversal, g| {
        let mut result = Vec::new();
        for vertex in traversal {
            result.extend(
                g.edges.
                    iter()
                    .filter(|e| e.source == vertex)
                    .map(|e| e.destination.clone())
            )
        }
        result
    })
}

pub fn expand_all<'a>() -> StepFn<'a> {
    Box::new(move |_, g| {
        g.vertices.clone()
    })
}

fn has<'a>(key: &'a str, value: &'a str) -> StepFn<'a> {
    Box::new(move | traversal, _| {
        traversal
            .into_iter()
            .filter(|v| v.properties.get(key) == Some(&value.to_string()))
            .collect::<Vec<Rc<Vertex>>>()
    })
}

fn r#in<'a>() -> StepFn<'a> {
    Box::new(move |traversal, g| {
        let mut result = Vec::new();
        for vertex in traversal {
            result.extend(
                g.edges.
                    iter()
                    .filter(|e| e.destination == vertex)
                    .map(|e| e.source.clone())
            )
        }
        result
    })
}