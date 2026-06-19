pub mod graph {
    use std::collections::HashMap;

    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<&'static str, &'static str>,
    }
    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }
        pub fn with_edges(self, edges: &[graph_items::edge::Edge]) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }
        pub fn with_nodes(self, nodes: &[graph_items::node::Node]) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }
        pub fn with_attrs(self, attrs: &[(&'static str, &'static str)]) -> Self {
            Graph {
                attrs: HashMap::from_iter(attrs.iter().copied()),
                ..self
            }
        }
        pub fn node(&self, node: &'static str) -> Option<graph_items::node::Node> {
            self.nodes.iter().find(|n| n.name == node).cloned()
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name: &'static str,
                pub attrs: HashMap<&'static str, &'static str>,
            }
            impl Node {
                pub fn new(name: &'static str) -> Self {
                    Node {
                        name,
                        attrs: HashMap::default(),
                    }
                }
                pub fn with_attrs(self, attrs: &[(&'static str, &'static str)]) -> Self {
                    Node {
                        attrs: HashMap::from_iter(attrs.iter().copied()),
                        ..self
                    }
                }
                pub fn attr(&self, key: &'static str) -> Option<&str> {
                    self.attrs.get(key).copied()
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                pub u: &'static str,
                pub v: &'static str,
                pub attrs: HashMap<&'static str, &'static str>,
            }
            impl Edge {
                pub fn new(u: &'static str, v: &'static str) -> Self {
                    Edge {
                        u,
                        v,
                        attrs: HashMap::default(),
                    }
                }
                pub fn with_attrs(self, attrs: &[(&'static str, &'static str)]) -> Self {
                    Edge {
                        attrs: HashMap::from_iter(attrs.iter().copied()),
                        ..self
                    }
                }
                pub fn attr(&self, key: &'static str) -> Option<&str> {
                    self.attrs.get(key).copied()
                }
            }
        }
    }
}
