use std::io;

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;

use crate::document::Document;
use crate::node::{NodeDataGetter, DefaultNodeData};


pub fn custom_parse_document<R, ND>(r: &mut R) -> Result<Document<ND>, String>
where R: io::Read, ND: NodeDataGetter + Default {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: false,
            ..Default::default()
        },
        ..Default::default()
    };

    let doc = Document::new();
    let result = html5ever::parse_document(doc, opts).from_utf8().read_from(r);
    match result {
        Result::Ok(doc) => {
            Ok(doc)
        }
        Result::Err(s) => {
            Err(s.to_string())
        }
    }
}

pub fn parse_document<R>(r: &mut R) -> Result<Document<DefaultNodeData>, String>
where R: io::Read {
    custom_parse_document::<R, DefaultNodeData>(r)
}