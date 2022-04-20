use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashSet, ptr, ops::DerefMut};

use html5ever::{QualName, Attribute, serialize::{Serialize, serialize, SerializeOpts, TraversalScope}};

use crate::iterator::{ChildrenIterator, DescendantIterator, DescendantOrder};


pub type Handle<ND> = Rc<RefCell<Node<ND>>>;
pub type WeakHandle<ND> = Weak<RefCell<Node<ND>>>;

pub struct NodeData<ND: NodeDataGetter> {
    parent: Option<WeakHandle<ND>>,
    prev_sibling: Option<WeakHandle<ND>>,
    next_sibling: Option<Handle<ND>>,
}

impl <ND: NodeDataGetter> Default for NodeData<ND> {
    fn default() -> NodeData<ND> {
        NodeData::<ND> { 
            parent: None,
            prev_sibling: None, 
            next_sibling: None,
        }
    }
}

pub struct DefaultNodeData {
    data: NodeData::<DefaultNodeData>,
}

impl Default for DefaultNodeData {
    fn default() -> DefaultNodeData {
        DefaultNodeData {
            data: Default::default(),
        }
    }
}

impl NodeDataGetter for DefaultNodeData {
    fn node_data(&self) -> &NodeData<Self> {
        &self.data
    }

    fn mut_node_data(&mut self) -> &mut NodeData<Self> {
        &mut self.data
    }
}

pub trait NodeDataGetter {
    fn node_data(&self) -> &NodeData<Self> where Self: Sized;
    fn mut_node_data(&mut self) -> &mut NodeData<Self> where Self: Sized;
}

pub enum Node<ND> where ND: NodeDataGetter {
    Text{
        data: ND,
        text: String,
    },
    Comment{
        data: ND,
        text: String,
    },
    Element{
        data: ND,
        name: QualName,
        attrs: Vec<Attribute>,
        first_child: Option<Handle<ND>>,
        last_child: Option<WeakHandle<ND>>,
        template_contents: Option<Handle<ND>>,
    },
    Document{
        data: ND,
        first_child: Option<Handle<ND>>,
        last_child: Option<WeakHandle<ND>>,
    },
    DocType{
        data: ND,
        name: String,
        public_id: String,
        system_id: String,
    },
}

pub fn new_handle<ND: NodeDataGetter>(node: Node<ND>) -> Handle<ND> {
    Rc::new(RefCell::new(node))
}


fn panic_is_not_text_node() -> ! {
    panic!("is not text node")
}

fn panic_is_not_comment_node() -> ! {
    panic!("is not comment node")
}

fn panic_is_not_element_node() -> ! {
    panic!("is not element node")
}

fn panic_not_exists_parent() -> ! {
    panic!("not exists parent")
}

fn format_qual_name(name: &QualName) -> String {
    let expand = name.expanded();
    if name.prefix.is_none() {
        expand.local.to_string()
    } else {
        name.prefix.as_ref().unwrap().to_string() + ":" + &expand.local
    }
}

impl<ND> PartialEq for Node<ND> where ND: NodeDataGetter + Default{
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.as_ptr(), other.as_ptr())
    }
}

fn upgrade_handle<ND>(node: Option<WeakHandle<ND>>) -> Option<Handle<ND>> 
where ND: NodeDataGetter + Default{
    node.map_or(None, |x| x.upgrade())
}

pub fn is_same_handle<ND>(node: &Handle<ND>, other: *const Node<ND>) -> bool
 where ND: NodeDataGetter + Default {
    ptr::eq(node.as_ptr(), other)
}

impl<ND> Node<ND> where ND: NodeDataGetter + Default {
    pub fn new_text_node(text: String) -> Self {
        Node::Text { 
            data: Default::default(),
            text, 
        }
    }

    pub fn new_comment_node(text: String) -> Self {
        Node::Comment { 
            data: Default::default(),
            text, 
        }
    }

    pub fn new_element_node(name: QualName, attrs: Vec<Attribute>) -> Self {
        Node::Element {
            data: Default::default(),
            name,
            attrs,
            first_child: None,
            last_child: None,
            template_contents: None,
        }
    }

    pub fn new_document_node() -> Self {
        Node::Document {
            data: Default::default(),
            first_child: None,
            last_child: None,
        }
    }

    pub fn new_doctype_node(name: String, public_id: String, system_id: String) -> Self {
        Node::DocType {
            data: Default::default(),
            name,
            public_id,
            system_id,
        }
    }

    pub fn template_contents(&mut self) -> Option<Handle<ND>> {
        match self {
            Node::Element{template_contents, ..} => template_contents.clone(),
            _ => panic_is_not_element_node()
        }
    }

    pub fn first_child(&self) -> Option<Handle<ND>> {
        match self {
            Node::Element{first_child, ..} => first_child.clone(),
            Node::Document{first_child, ..} => first_child.clone(),
            _ => panic_is_not_element_node()
        }
    }

    pub fn last_child(&self) -> Option<Handle<ND>> {
        match self {
            Node::Element{last_child, ..} => upgrade_handle(last_child.clone()),
            Node::Document{last_child, ..} => upgrade_handle(last_child.clone()),
            _ => panic_is_not_element_node()
        }
    }

    fn node_data(&self) -> &NodeData<ND> {
        self.custom_node_data().node_data()
    }

    fn mut_node_data(&mut self) -> &mut NodeData<ND> {
        self.mut_custom_node_data().mut_node_data()
    }

    pub fn custom_node_data(&self) -> &ND {
        match self {
            Node::Text{data, ..} => {
                data
            },
            Node::Comment {data, ..} => {
                data
            },
            Node::Element {data, ..} => {
                data
            },
            Node::Document {data, ..} => {
                data
            },
            Node::DocType {data, ..} => {
                data
            },
        }
    }

    pub fn mut_custom_node_data(&mut self) -> &mut ND {
        match self {
            Node::Text{data, ..} => {
                data
            },
            Node::Comment {data, ..} => {
                data
            },
            Node::Element {data, ..} => {
                data
            },
            Node::Document {data, ..} => {
                data
            },
            Node::DocType {data, ..} => {
                data
            },
        }
    }

    pub fn prev_sibling(&self) -> Option<Handle<ND>> {
        upgrade_handle(self.node_data().prev_sibling.clone())
    }

    pub fn next_sibling(&self) -> Option<Handle<ND>> {
        self.node_data().next_sibling.clone()
    }

    pub fn name(&self) -> &QualName {
        if let Node::Element{name, ..} = self {
            return name;
        }
        panic_is_not_text_node()
    }

    pub fn tag_name(&self) -> String {
        if let Node::Element{name, ..} = self {
            format_qual_name(name)
        } else {
            panic_is_not_text_node()
        }
    }

    pub fn text(&self) -> String {
        if let Node::Text{text, ..} = self {
            return text.clone();
        }
        panic_is_not_text_node()
    }

    pub fn comment(&self) -> String {
        if let Node::Comment{text, ..} = self {
            return text.clone();
        }
        panic_is_not_comment_node()
    }

    pub fn title(&self) -> Option<String> {
        self.head().borrow().children().find(|child| 
            child.borrow().is_element_node() && child.borrow().tag_name() == "title")
            .map(|node| node.borrow().children_text())
    }

    pub fn ld_json(&self) -> Option<String> {
        self.head().borrow().children().find(|child| {
                child.borrow().is_element_node() && child.borrow().tag_name() == "script" 
                    && child.borrow().attr("type") == Some("application/ld+json".to_string())
            }).map(|node| node.borrow().children_text())
    }

    pub fn is_text_node(&self) -> bool {
        if let Node::Text{..} = self {
            return true;
        }
        return false;
    }

    pub fn is_element_node(&self) -> bool {
        if let Node::Element{..} = self {
            return true;
        }
        return false;
    }

    pub fn has_children(&self) -> bool {
        match self {
            Node::Element{..} | Node::Document {..} =>  true,
            _ => false
        }
    }

    pub fn is_none_tag(&self) -> bool {
        vec!["script", "style"].contains(&self.tag_name().as_ref())
    }

    pub fn is_inline_tag(&self) -> bool {
        vec![
            "a", "abbr", "acronym", "b", "bdo", "big", "br", "button", "cite",
            "code", "dfn", "em", "i", "img", "input", "kbd", "label", "map",
            "object", "output", "q", "samp", "script", "select", "small",
            "span", "strong", "sub", "sup", "textarea", "time", "tt", "u", "var",
        ].contains(&self.tag_name().as_ref())
    }

    pub fn append_child(&mut self, child: &Handle<ND>) {
        let (first_child, last_child) = match self {
            Node::Element{first_child, last_child, ..} => {
                (first_child, last_child)
            },
            Node::Document{first_child, last_child, ..} => {
                (first_child, last_child)
            },
            _ => panic_is_not_element_node()
        };
        let owner_last_child = upgrade_handle(last_child.clone());
        if owner_last_child.is_none() {
            *first_child = Some(child.clone());
        } else {
            let owner_last_child = owner_last_child.unwrap();
            owner_last_child.as_ref().borrow_mut().mut_node_data().next_sibling = Some(child.clone());
            child.as_ref().borrow_mut().mut_node_data().prev_sibling = Option::Some(Rc::downgrade(&owner_last_child));
        }
        *last_child = Some(Rc::downgrade(child));
    }

    pub fn append_before_sibling(&mut self, node: &Handle<ND>) {
        let parent = self.parent();
        if parent.is_none() {
            panic_not_exists_parent()
        }
        let parent = parent.unwrap();
        let prev = self.prev_sibling();
        if prev.is_some() {
            let prev = prev.unwrap();
            node.as_ref().borrow_mut().mut_node_data().next_sibling = prev.as_ref().borrow_mut().next_sibling().clone();
            prev.as_ref().borrow_mut().mut_node_data().next_sibling = Some(node.clone());
        } else {
            let mut mut_parent = parent.as_ref().borrow_mut();
            let first_child = match &mut *mut_parent {
                Node::Element{first_child, ..} => {
                    first_child
                },
                Node::Document{first_child, ..} => {
                    first_child
                },
                _ => panic_is_not_element_node(),
            };
            node.as_ref().borrow_mut().mut_node_data().next_sibling = first_child.clone();
            *first_child = Some(node.clone());
        }
        node.as_ref().borrow_mut().set_parent(&parent);
        self.mut_node_data().prev_sibling = Some(Rc::downgrade(node));
    }

    pub fn parent(&self) -> Option<Handle<ND>> {
        upgrade_handle(self.node_data().parent.clone())
    }

    pub fn set_parent(&mut self, parent: &Handle<ND>) {
        self.mut_node_data().parent = Some(Rc::downgrade(parent));
    }

    pub fn attrs(&self) -> &Vec<Attribute> {
        match self {
            Node::Element {attrs, ..} => {
                attrs
            },
            _ => panic_is_not_element_node(),
        }
    }

    pub fn mut_attrs(&mut self) -> &mut Vec<Attribute> {
        match self {
            Node::Element {attrs, ..} => {
                attrs
            },
            _ => panic_is_not_element_node(),
        }
    }

    pub fn pop_attr(&mut self, name: &str) -> Option<String> {
        if let Some(index) = self.attrs().iter().position(|attr|attr.name.local.as_ref() == name) {
            Some(self.mut_attrs().swap_remove(index).value.to_string())
        } else {
            None
        }
    }

    pub fn attr(&self, name: &str) -> Option<String> {
        self.attrs().iter()
            .find(|attr| attr.name.local.as_ref() == name)
            .map(|attr| attr.value.to_string())
    }

    pub fn add_attrs_if_missing(&mut self, attrs: Vec<Attribute>) {
        let mut_attrs = self.mut_attrs();
        let existing_names = mut_attrs.iter().map(|e| e.name.clone()).collect::<HashSet<_>>();
        mut_attrs.extend(
            attrs
                .into_iter()
                .filter(|attr| !existing_names.contains(&attr.name)),
        );
    }

    pub fn remove_from_parent(&mut self) {
        let parent = self.parent();
        if parent.is_none() {
            panic_not_exists_parent()
        }

        let mut parent = (**parent.as_ref().unwrap()).borrow_mut();
        let parent = &mut *parent;

        let size = parent.children().count();
        let (first_child, last_child) = match parent {
            Node::Element {first_child, last_child, ..} => {
                (first_child, last_child)
            },
            Node::Document {first_child, last_child, ..} => {
                (first_child, last_child)
            },
            _ => panic_is_not_element_node(),
        };

        if size == 1 {
            *first_child = None;
            *last_child = None;
        } else {
            let owner_last_child = upgrade_handle(last_child.clone()).unwrap();
            let owner_first_child = first_child.clone().unwrap();
            if is_same_handle(&owner_first_child, self) {
                *first_child = owner_first_child.as_ref().borrow_mut().next_sibling();
            } else if is_same_handle(&owner_last_child, self) {
                *last_child = owner_last_child.as_ref().borrow_mut().node_data().prev_sibling.clone();
            } else {
                self.prev_sibling().unwrap().as_ref().borrow_mut().mut_node_data().next_sibling = self.next_sibling();
                self.next_sibling().unwrap().as_ref().borrow_mut().mut_node_data().prev_sibling = self.node_data().prev_sibling.clone();
            }
        }
        *self.mut_node_data() = Default::default();
    }

    pub fn children(&self) -> ChildrenIterator<ND> {
        ChildrenIterator{
            node: self.first_child(),
        }
    }

    pub fn descendants(&self) -> DescendantIterator<ND, impl Fn(&Node<ND>) -> bool> {
        self.descendants_skip(|_| false)
    }

    pub fn postorder_descendants(&self) -> DescendantIterator<ND, impl Fn(&Node<ND>) -> bool> {
        self.postorder_descendants_skip(|_| false)
    }

    pub fn descendants_skip<F>(&self, f: F) -> DescendantIterator<ND, F> where F: Fn(&Node<ND>) -> bool {
        DescendantIterator::new(self, DescendantOrder::PreOrder, f)
    }

    pub fn postorder_descendants_skip<F>(&self, f: F) -> DescendantIterator<ND, F>  where F: Fn(&Node<ND>) -> bool {
        DescendantIterator::new(self, DescendantOrder::PostOrder, f)
    }

    pub fn document_node(&self) -> Handle<ND> {
        match self {
            Node::Document{..} => {
                let a = self.first_child().unwrap();
                let b = a.as_ref().borrow().parent().unwrap();
                b
            }
            _ => {
                self.parent().unwrap().as_ref().borrow().document_node()
            }
        }
    }

    pub fn root(&self) -> Handle<ND> {
        self.document_node().as_ref().borrow().children().find(|child| child.as_ref().borrow().is_element_node()).unwrap()
    }

    pub fn body(&self) -> Handle<ND> {
        self.root().as_ref().borrow().children().find(|child| {
            let child = child.as_ref().borrow();
            child.is_element_node() && child.tag_name() == "body"
        }).unwrap()
    }

    pub fn head(&self) -> Handle<ND> {
        self.root().as_ref().borrow().children().find(|child| {
            let child = child.as_ref().borrow();
            child.is_element_node() && child.tag_name() == "head"
        }).unwrap()
    }

    pub fn children_text(&self) -> String {
        match self {
            Node::Text{text, ..}  => text.clone(),
            Node::Element{..} => {
                self.children().filter(|child| child.borrow().is_text_node())
                    .map(|child| child.borrow().text())
                    .fold(String::new(), |mut s, v| {s.push_str(&v); s})
            }
            _ => "".to_string()
        }
    }

    pub fn descendants_text_skip<F>(&self, f: F) -> String where F: Fn(&Node<ND>) -> bool {
        match self {
            Node::Text{text, ..}  => text.clone(),
            Node::Element{..} => {
                if f(self) {
                    return "".to_string();
                }
                let mut block = true;
                self.descendants_skip(f)
                    .fold(String::new(), |mut s, node| {
                        if node.borrow().is_text_node() {
                            /*
                                let parent = node.borrow().parent().unwrap();
                                if !parent.borrow().is_inline_tag() && !s.is_empty() {
                                    s.push(' ');
                                }
                            */
                            s.push_str(&node.borrow().text());
                            block = false;
                        } else if node.borrow().is_element_node() && !node.borrow().is_inline_tag() && !block {
                            block = true;
                            s.push('\n');
                        }
                        s
                    })
            }
            _ => "".to_string()
        }
    }

    pub fn descendants_text(&self) -> String {
        self.descendants_text_skip(|node| node.is_element_node() && node.is_none_tag())
    }

    pub fn inner_html(&self) -> String {
        let mut w = vec![];
        serialize(&mut w, self, SerializeOpts{
            scripting_enabled: true,
            traversal_scope: TraversalScope::ChildrenOnly(None),
            create_missing_parent: false,
        }).unwrap();
        String::from_utf8(w).unwrap()
    }

    pub fn outer_html(&self) -> String {
        let mut w = vec![];
        serialize(&mut w, self, SerializeOpts{
            scripting_enabled: true,
            traversal_scope: TraversalScope::IncludeNode,
            create_missing_parent: true,
        }).unwrap();
        String::from_utf8(w).unwrap()
    }

    pub fn as_ptr(&self) -> *const Node<ND> {
        self as *const Node<ND>
    }

    pub fn trace<F>(&mut self, f: F) where F: Fn(&mut Node<ND>) {
        f(self);
        self.descendants().for_each(|child| f(child.borrow_mut().deref_mut()));
    }

    pub fn postorder_trace<F>(&mut self, f: F) where F: Fn(&mut Node<ND>) {
        self.postorder_descendants().for_each(|child| f(child.borrow_mut().deref_mut()));
        f(self);
    }
}

impl<ND> Serialize for Node<ND> where ND: NodeDataGetter + Default{
    fn serialize<S>(&self, serializer: &mut S, traversal_scope: html5ever::serialize::TraversalScope) -> std::io::Result<()>
    where
        S: html5ever::serialize::Serializer {
            match self {
                Node::Text { text , ..} => {
                    serializer.write_text(text)
                },
                Node::Comment { text, ..} => {
                    serializer.write_comment(text)
                },
                Node::Element { name, attrs, ..} => {
                    let include_node = traversal_scope == TraversalScope::IncludeNode;
                    if include_node {
                        serializer.start_elem(name.clone(), attrs.iter().map(|at| (&at.name, &at.value[..])))?;
                    }
                    self.children().fold(Ok(()),|v, child| {
                        v?;
                        child.as_ref().borrow().serialize(serializer, TraversalScope::IncludeNode)
                    })?;
                    if include_node {
                        serializer.end_elem(name.clone())?;
                    }
                    Ok(())
                },
                Node::Document {..} => {
                    self.children().fold(Ok(()),|v, child| {
                        v?;
                        child.as_ref().borrow().serialize(serializer, TraversalScope::IncludeNode)
                    })
                },
                Node::DocType { name, ..} => {
                    serializer.write_doctype(name)
                },
            }
    }
}