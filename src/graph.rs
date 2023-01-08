use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use uuid::Uuid;
use crate::traversal::{expand_all, find_one, Traversal};

/// A `Vertex` have an ID, an optional label and a list of properties.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub id: Uuid,
    pub label: Option<String>,
    pub properties: HashMap<String, String>
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            label: None,
            properties: HashMap::new()
        }
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // For each properties of the Vertex, we want to build
        // a list of string in the following format: `key: value`
        let properties = self.properties
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join(", ");

        // If a label is defined, it will be printed instead of the ID
        let id = self.label.clone().unwrap_or(self.id.to_string());

        write!(f, "[{}] {{ {} }}", id, properties)
    }
}

/// `VertexBuilder` is used to build a Vertex from the graph.
pub struct VertexBuilder<'a> {
    graph: &'a mut Graph,
    label: Option<String>,
    properties: HashMap<String, String>
}

impl<'a> VertexBuilder<'a> {
    #[allow(dead_code)]
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> Rc<Vertex> {
        let vertex = Rc::from(Vertex {
            label: self.label,
            properties: self.properties,
            ..Vertex::default()
        });
        self.graph.vertices.push(vertex.clone());
        vertex
    }
}

/// An `Edge` has three attributes : a label,
/// a source Vertex and a destination Vertex.
#[derive(Debug, Clone)]
pub struct Edge {
    pub label: String,
    pub source: Rc<Vertex>,
    pub destination: Rc<Vertex>
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} --[{}]--> {}", self.source, self.label.to_uppercase(), self.destination)
    }
}

/// `EdgeBuilder` is used to build an Edge from the graph.
pub struct EdgeBuilder<'a> {
    graph: &'a mut Graph,
    label: String,
    source: Option<Rc<Vertex>>,
    destination: Option<Rc<Vertex>>,
}

impl<'a> EdgeBuilder<'a> {
    pub fn source(mut self, from: &Rc<Vertex>) -> Self {
        self.source = Some(from.clone());
        self
    }

    pub fn destination(mut self, to: &Rc<Vertex>) -> Self {
        self.destination = Some(to.clone());
        self
    }

    pub fn build(self) -> Edge {
        let from = self.source.expect("missing 'from' vertex");
        let to = self.destination.expect("missing 'to' vertex");

        let edge = Edge {
            label: self.label,
            source: from,
            destination: to,
        };

        self.graph.edges.push(edge.clone());
        edge
    }
}

/// A `Graph` is simply a struct that holds a vector of `Vertex`
/// and a vector of `Edge`
#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: Vec<Rc<Vertex>>,
    pub edges: Vec<Edge>
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_v(&mut self) -> VertexBuilder {
        VertexBuilder {
            graph: self,
            label: None,
            properties: HashMap::new()
        }
    }

    pub fn add_e(&mut self, label: &str) -> EdgeBuilder {
        EdgeBuilder {
            graph: self,
            label: label.to_string(),
            source: None,
            destination: None
        }
    }

    pub fn e(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn v(&self, vertex: Option<Rc<Vertex>>) -> Traversal {
        let steps = match vertex {
            None => vec![expand_all()],
            Some(v) => vec![find_one(v)]
        };
        Traversal::new(self, steps)
    }
}