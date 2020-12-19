use std::fmt;

#[derive(PartialEq)]
pub enum Node {
    In,
    Out,
    Node(String),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.do_fmt(f)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.do_fmt(f)
    }
}

impl Node {
    pub fn new_node(name: &str) -> Self {
        Node::Node(name.to_string())
    }

    pub fn get_id(&self) -> &String {
        match self {
            Node::Node(ref id) => id,
            _ => panic!("Not a named node: {}", self),
        }
    }

    fn do_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::In => write!(f, "<IN>"),
            Node::Out => write!(f, "<OUT>"),
            Node::Node(ref id) => write!(f, "Node#{}", id),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Port {
    pub node: Node,
    pub port: u32,
}

impl Port {
    pub fn new(node: Node, port: u32) -> Self {
        Port { port, node }
    }

    pub fn named_port(node_name: &str, port: u32) -> Self {
        Port {
            node: Node::new_node(node_name),
            port,
        }
    }
}
