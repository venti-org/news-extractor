use crate::{NodeDataGetter, Handle, Node, is_same_handle};

pub struct ChildrenIterator<ND> where ND: NodeDataGetter + Default{
    pub(crate) node: Option<Handle<ND>>,
}

impl<ND> Iterator for ChildrenIterator<ND> where ND: NodeDataGetter + Default {
    type Item = Handle<ND>;

    fn next(&mut self) -> Option<Self::Item> {
        //self.node = ret.unwrap().next_sibling().clone();
        let ret = self.node.clone();
        self.node = self.node.clone().map_or(None, |x| x.as_ref().borrow().next_sibling());
        ret
    }
}

pub enum DescendantOrder {
    PreOrder,
    PostOrder,
}

pub struct DescendantIterator<ND, F> 
where ND: NodeDataGetter + Default, F: Fn(&Node<ND>) ->bool{
    pub(crate) start: Option<*const Node<ND>>,
    pub(crate) node: Option<Handle<ND>>,
    pub(crate) order: DescendantOrder, 
    pub(crate) skip_func: F,
}

impl<ND, F> DescendantIterator<ND, F> 
where ND: NodeDataGetter + Default, F: Fn(&Node<ND>) ->bool {
    pub fn new(node: &Node<ND>, order: DescendantOrder, skip_func: F) -> DescendantIterator<ND, F> {
        let iter = match node.first_child() {
            Some(child) => DescendantIterator{
                start: Some(node as * const Node<ND>),
                node: if let DescendantOrder::PreOrder = order {
                    DescendantIterator::<ND, F>::find_next_node(child.clone(), &skip_func)
                } else {
                    DescendantIterator::<ND, F>::find_postorder_node(child.clone(), &skip_func)
                },
                order,
                skip_func,
            },
            None => DescendantIterator{
                start: None,
                node: None,
                order, 
                skip_func,
            }
        };
        iter
    }

    fn find_postorder_node(mut node: Handle<ND>, skip_func: &F) -> Option<Handle<ND>> {
        loop {
            if skip_func(&node.borrow()) {
                return None;
            }
            if !node.borrow().has_children() {
                return Some(node);
            }
            let child = node.borrow().first_child();
            match child {
                Some(child) => {
                    node = child;
                },
                None => {
                    return Some(node);
                }
            }
        }
    }

    fn find_next_node(mut next: Handle<ND>, skip_func: &F) -> Option<Handle<ND>> {
        loop {
            if !skip_func(&next.borrow()) {
                return Some(next);
            }
            let tmp = next.borrow().next_sibling()?;
            next = tmp;
        }
    }

    fn preorder_next(&mut self) -> Option<Handle<ND>> {
        self.node.clone().map_or(None, |node| {
            if node.borrow().has_children() {
                let child = node.borrow().first_child();
                if child.is_some() {
                    let result = DescendantIterator::<ND, F>::find_next_node(child.unwrap(), &self.skip_func);
                    if result.is_some() {
                        return result;
                    }
                }
            }
            let start = self.start.unwrap();
            let mut parent = node;
            loop {
                let next = parent.borrow().next_sibling();
                if next.is_some() {
                    let result = DescendantIterator::<ND, F>::find_next_node(next.unwrap(), &self.skip_func);
                    if result.is_some() {
                        return result;
                    }
                }
                let tmp = parent.borrow().parent()?;
                parent = tmp;
                if is_same_handle(&parent, start) {
                    return None;
                }
            }
        })
    }

    fn postorder_next(&mut self) -> Option<Handle<ND>> {
        self.node.clone().map_or(None, |node| {
            let next = node.borrow().next_sibling();
            match next {
                Some(next) => {
                    DescendantIterator::<ND, F>::find_postorder_node(next, &self.skip_func)
                },
                None => {
                    let start = self.start.unwrap();
                    let parent = node.borrow().parent()?;
                    if is_same_handle(&parent, start) {
                        return None;
                    }
                    Some(parent)
                },
            }
        })
    }
}

impl<ND, F> Iterator for DescendantIterator<ND, F> 
where ND: NodeDataGetter + Default, F: Fn(&Node<ND>) ->bool{
    type Item = Handle<ND>;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.node.clone();
        self.node = match self.order {
            DescendantOrder::PreOrder => self.preorder_next(),
            _ => self.postorder_next(),
        };
        ret
    }
}
