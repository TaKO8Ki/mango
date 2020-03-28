use crate::domain::entity::dom;
use crate::interfaces::controllers::parser::html;

#[test]
fn parse_test() {
    let html: &str = "
                    <body>
                    <div>
                        <h1>Example Domain</h1>
                        <p>This domain is for use in illustrative examples in documents. You may use this
                        domain in literature without prior coordination or asking for permission.</p>
                        <p><a href=\"https://www.iana.org/domains/example\">More information...</a></p>
                    </div>
                    </body>
                    ";
    let node = html::parse(html.to_string());
    assert_eq!(node.children.len(), 1);
    match node.node_type {
        dom::NodeType::Element(element_data) => assert_eq!(element_data.tag_name, "body"),
        dom::NodeType::Text(text) => assert_eq!(text, ""),
    }
}
