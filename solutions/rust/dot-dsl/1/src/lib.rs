pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Self { nodes: vec![], edges: vec![], attrs: HashMap::new() }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }
        
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }

            self
        }
        
        pub fn node(&self, data: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.data == data)
        }
    }
    
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                attrs: HashMap<String, String>
            }
            
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self { from: from.to_string(), to: to.to_string(), attrs: HashMap::new() }
                }
                
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    
                    self
                }
                
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }
        
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub data: String,
                attrs: HashMap<String, String>
            }
            
            impl Node {
                pub fn new(data: &str) -> Self {
                    Self { data: data.to_string(), attrs: HashMap::new() }
                }
                
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }
    }
}