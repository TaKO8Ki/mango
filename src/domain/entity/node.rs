#[derive(Debug)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
  Text(String),
  Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMap,
}

pub type AttrMap = std::collections::HashMap<String, String>;
