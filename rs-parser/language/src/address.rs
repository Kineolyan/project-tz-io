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
pub struct Port<Slot> {
    pub node: Node,
    pub port: Slot,
}

impl<Slot> Port<Slot> {
    pub fn new(node: Node, port: Slot) -> Self {
        Port { port, node }
    }

    pub fn named_port(node_name: &str, port: Slot) -> Self {
        Port {
            node: Node::new_node(node_name),
            port,
        }
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub struct InputSlot(u8);

impl std::convert::From<u8> for InputSlot {
    fn from(value: u8) -> Self {
        InputSlot(value)
    }
}

impl fmt::Display for InputSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub struct OutputSlot(u8);

impl std::convert::From<u8> for OutputSlot {
    fn from(value: u8) -> Self {
        OutputSlot(value)
    }
}

impl fmt::Display for OutputSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
