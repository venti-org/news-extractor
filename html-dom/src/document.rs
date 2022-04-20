use std::borrow::Cow;

use html5ever::Attribute;
use html5ever::ExpandedName;
use html5ever::QualName;
use html5ever::tendril::StrTendril;
use html5ever::tree_builder::ElementFlags;
use html5ever::tree_builder::NodeOrText;
use html5ever::tree_builder::QuirksMode;
use html5ever::tree_builder::TreeSink;
use crate::node::Node;
use crate::node::Handle;
use crate::node::NodeDataGetter;
use crate::node::new_handle;


pub struct Document<ND> where ND: NodeDataGetter + Default {
    pub document: Handle<ND>,
    pub errors: Vec<String>,
}

impl <ND> Document<ND> where ND: NodeDataGetter + Default {
    pub fn new() -> Document<ND> {
        Document::<ND> {
            document: new_handle(Node::new_document_node()),
            errors: vec!(),
        }
    }
}

impl<ND> TreeSink for Document<ND> where ND: NodeDataGetter + Default {
    type Handle = Handle<ND>;

    /// The overall result of parsing.
    ///
    /// This should default to Self, but default associated types are not stable yet.
    /// [rust-lang/rust#29661](https://github.com/rust-lang/rust/issues/29661)
    type Output = Self;

    /// Consume this sink and return the overall result of parsing.
    ///
    /// TODO:This should default to `fn finish(self) -> Self::Output { self }`,
    /// but default associated types are not stable yet.
    /// [rust-lang/rust#29661](https://github.com/rust-lang/rust/issues/29661)
    fn finish(self) -> Self {
        self
    }

    /// Signal a parse error.
    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg.to_string());
    }

    /// Get a handle to the `Document` node.
    fn get_document(&mut self) -> Self::Handle {
        self.document.clone()
    }

    /// What is the name of this element?
    ///
    /// Should never be called on a non-element node;
    /// feel free to `panic!`.
    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        unsafe { 
            &*target.as_ptr()
        }.name().expanded()
    }

    /// Create an element.
    ///
    /// When creating a template element (`name.ns.expanded() == expanded_name!(html "template")`),
    /// an associated document fragment called the "template contents" should
    /// also be created. Later calls to self.get_template_contents() with that
    /// given element return it.
    /// See [the template element in the whatwg spec][whatwg template].
    ///
    /// [whatwg template]: https://html.spec.whatwg.org/multipage/#the-template-element
    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        new_handle(Node::new_element_node(name, attrs))
    }

    /// Create a comment node.
    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        new_handle(Node::new_comment_node(text.to_string()))
    }

    /// Create a Processing Instruction node.
    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> Self::Handle {
        new_handle(Node::new_comment_node(target.to_string() + " " + &data))
    }

    /// Append a node as the last child of the given node.  If this would
    /// produce adjacent sibling text nodes, it should concatenate the text
    /// instead.
    ///
    /// The child node will not already have a parent.
    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(node) => {
                parent.borrow_mut().append_child(&node);
                node.borrow_mut().set_parent(parent);
            },
            NodeOrText::AppendText(text) => {
                let node = new_handle(Node::new_text_node(text.to_string()));
                parent.borrow_mut().append_child(&node);
                node.borrow_mut().set_parent(parent);
            },
        }
    }

    /// When the insertion point is decided by the existence of a parent node of the
    /// element, we consider both possibilities and send the element which will be used
    /// if a parent node exists, along with the element to be used if there isn't one.
    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let has_parent = element.borrow_mut().parent().is_some();

        if has_parent {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    /// Append a `DOCTYPE` element to the `Document` node.
    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let node = new_handle(Node::new_doctype_node(name.to_string(), public_id.to_string(), system_id.to_string()));
        self.document.borrow_mut().append_child(&node);
        node.borrow_mut().set_parent(&self.document);
    }

    /// Mark a HTML `<script>` as "already started".
    fn mark_script_already_started(&mut self, _node: &Self::Handle) {}

    /// Indicate that a node was popped off the stack of open elements.
    fn pop(&mut self, _node: &Self::Handle) {}

    /// Get a handle to a template's template contents. The tree builder
    /// promises this will never be called with something else than
    /// a template element.
    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        target.borrow_mut().template_contents().unwrap()
    }

    /// Do two handles refer to the same node?
    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    /// Set the document's quirks mode.
    fn set_quirks_mode(&mut self, _mode: QuirksMode) {
    }

    /// Append a node as the sibling immediately before the given node.
    ///
    /// The tree builder promises that `sibling` is not a text node.  However its
    /// old previous sibling, which would become the new node's previous sibling,
    /// could be a text node.  If the new node is also a text node, the two should
    /// be merged, as in the behavior of `append`.
    ///
    /// NB: `new_node` may have an old parent, from which it should be removed.
    fn append_before_sibling(&mut self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>) {
        match new_node {
            NodeOrText::AppendNode(node) => {
                sibling.borrow_mut().append_before_sibling(&node);
            },
            NodeOrText::AppendText(text) => {
                let node = new_handle(Node::new_text_node(text.to_string()));
                sibling.borrow_mut().append_before_sibling(&node);
            },
        }
    }

    /// Add each attribute to the given element, if no attribute with that name
    /// already exists. The tree builder promises this will never be called
    /// with something else than an element.
    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<Attribute>) {
        target.borrow_mut().add_attrs_if_missing(attrs);
    }

    /// Associate the given form-associatable element with the form element
    fn associate_with_form(
        &mut self,
        _target: &Self::Handle,
        _form: &Self::Handle,
        _nodes: (&Self::Handle, Option<&Self::Handle>),
    ) {
    }

    /// Detach the given node from its parent.
    fn remove_from_parent(&mut self, target: &Self::Handle) {
        target.borrow_mut().remove_from_parent()
    }

    /// Remove all the children from node and append them to new_parent.
    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        node.borrow_mut().remove_from_parent();
        new_parent.borrow_mut().append_child(node);
    }

    /// Returns true if the adjusted current node is an HTML integration point
    /// and the token is a start tag.
    fn is_mathml_annotation_xml_integration_point(&self, _handle: &Self::Handle) -> bool {
        false
    }

    /// Called whenever the line number changes.
    fn set_current_line(&mut self, _line_number: u64) {}

    fn complete_script(&mut self, _node: &Self::Handle) -> html5ever::tree_builder::NextParserState {
        html5ever::tree_builder::NextParserState::Continue
    }
}