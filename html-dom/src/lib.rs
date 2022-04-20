mod node;
mod iterator;
mod document;
mod parser;

pub use parser::parse_document;
pub use parser::custom_parse_document;
pub use node::NodeData;
pub use node::NodeDataGetter;
pub use node::Handle;
pub use node::Node;
pub use document::Document;
pub use node::is_same_handle;

#[cfg(test)]
mod tests {

use html5ever::tree_builder::TreeSink;

use crate::{parse_document, custom_parse_document, node::{NodeData, NodeDataGetter}};

const HTML: &str = r#"<!DOCTYPE html><html><head>
<style>
.abc {
    max-height: 100px;
    width: auto;
}
</style>
<title> fuck you </title>
</head>
<body>
<div class=""> <img class="abc&quot;" src="layout.gif" width="512" height="512">
</div>

</body></html>"#;

#[test]
fn test_outer_html() {
    let mut m = HTML.as_bytes();
    let mut doc = match parse_document(&mut m) {
        Ok(doc) => doc,
        Err(err) => panic!("{}", err)
    };

    let outer_html = doc.get_document().borrow().outer_html();
    assert_eq!(outer_html.len(), HTML.len());
}

struct CustomNodeData {
    data: NodeData<Self>,
    src: String,
}

impl NodeDataGetter for CustomNodeData {
    fn node_data(&self) -> &NodeData<Self> where Self: Sized {
        &self.data
    }

    fn mut_node_data(&mut self) -> &mut NodeData<Self> where Self: Sized {
        &mut self.data
    }
}

impl Default for CustomNodeData {
    fn default() -> Self {
        Self { 
            data: Default::default(), 
            src: Default::default(),
        }
    }
}

#[test]
fn test_custom_parser() {
    let mut doc = match custom_parse_document::<_, CustomNodeData>(&mut HTML.as_bytes()) {
        Ok(doc) => doc,
        Err(err) => panic!("{}", err)
    };

    doc.get_document().borrow_mut().trace(|node| {
        if node.is_element_node() {
            if let Some(src) = node.attr("src") {
                assert_eq!(src, "layout.gif");
                node.mut_custom_node_data().src = src;
            }
        }
    });

    let outer_html = doc.get_document().borrow().outer_html();
    assert_eq!(outer_html.len(), HTML.len());
}
}