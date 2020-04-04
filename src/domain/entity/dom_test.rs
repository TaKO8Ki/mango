use crate::domain::entity::dom;
use std::collections::HashMap;

#[test]
fn test_text() {
    let data = "fizz buzz";
    let wanted = data;
    let node = dom::text(data.to_string());
    match node.node_type {
        dom::NodeType::Text(text) => assert_eq!(text, wanted.to_string()),
        dom::NodeType::Element(_) => (),
    }
}

#[test]
fn test_elem() {
    let name = "body";
    let mut attrs: dom::AttrMap = HashMap::new();
    attrs.insert(
        "http".to_string(),
        "https://www.iana.org/domains/example".to_string(),
    );
    let children: Vec<dom::Node> = Vec::new();
    let node = dom::elem(name.to_string(), attrs, children);
    match node.node_type {
        dom::NodeType::Text(_) => (),
        dom::NodeType::Element(element_data) => assert_eq!(element_data.tag_name, name),
    }
}
