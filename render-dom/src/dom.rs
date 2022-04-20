use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use html_dom::{NodeData, NodeDataGetter, Document};
use html_dom::custom_parse_document;

use crate::style::{StyleInfo, parse_style_info};


pub struct Vision{
    pub width: i32,
    pub height: i32,
    pub xpos: i32,
    pub ypos: i32,
    pub visible: i32,
}

impl Default for Vision {
    fn default() -> Self {
        Self{
            width: -1,
            height: -1,
            xpos: -1,
            ypos: -1,
            visible: -1,
        }
    }
}

pub struct RenderNodeData {
    pub id: u32,
    pub data: NodeData<Self>,
    pub vision: Vision,
    pub style: StyleInfo,
}

impl NodeDataGetter for RenderNodeData {
    fn node_data(&self) -> &NodeData<Self> where Self: Sized {
        &self.data
    }

    fn mut_node_data(&mut self) -> &mut NodeData<Self> where Self: Sized {
        &mut self.data
    }
}

impl RenderNodeData {
    pub fn visible(&self) -> bool {
        self.vision.visible > 0
    }
}

impl Default for RenderNodeData {
    fn default() -> Self {
        Self {
            id: 0,
            data: Default::default(),
            vision: Default::default(),
            style: Default::default(),
        }
    }
}

fn parse_vision(s: &str) -> Option<Vision> {
    let items = s.split(';').into_iter().flat_map(|x| x.parse::<i32>()).collect::<Vec<_>>();
    if items.len() == 5 {
        Some(Vision{
            width: items[0],
            height: items[1],
            xpos: items[2],
            ypos: items[3],
            visible: items[4],
        })
    } else {
        None
    }
}

pub fn parse_document<R>(r: &mut R) -> Result<Document<RenderNodeData>, String>
where R: io::Read {
    let doc = custom_parse_document::<R, RenderNodeData>(r)?;
    let id = Rc::new(RefCell::new(0 as u32));
    doc.document.borrow_mut().trace(|node| {
        *id.borrow_mut() += 1;
        node.mut_custom_node_data().id = *id.borrow();
        if node.is_element_node() {
            if let Some(vision) = node.pop_attr("surface_vision_info") {
                if let Some(vision) = parse_vision(&vision) {
                    node.mut_custom_node_data().vision = vision;
                }
            }
            if let Some(style_info) = node.pop_attr("dom_style_info") {
                parse_style_info(&style_info, &mut node.mut_custom_node_data().style);
            }
        }
    });
    Ok(doc)
}