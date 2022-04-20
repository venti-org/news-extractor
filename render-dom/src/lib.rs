mod dom;
mod style;

#[macro_use]
extern crate num_derive;

pub use dom::parse_document;
pub use dom::RenderNodeData;
pub type Handle = html_dom::Handle<RenderNodeData>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
