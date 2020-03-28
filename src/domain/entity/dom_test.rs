use crate::domain::entity::dom;

#[test]
fn text_test() {
    let data = "fizz buzz";
    let wanted = data;
    let node = dom::text(data.to_string());
    match node.node_type {
        dom::NodeType::Text(text) => assert_eq!(text, wanted.to_string()),
        dom::NodeType::Element(_) => (),
    }
}
