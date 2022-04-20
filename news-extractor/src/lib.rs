use std::io;

use render_dom::parse_document;

mod parser;
pub use parser::Feature;


pub fn parse_html<R>(url: String, r: &mut R) -> Result<Feature, String> 
 where R: io::Read {
    let doc = parse_document(r)?;
    let borrow = doc.document.borrow();
    parser::parse_tree(url, borrow.root())
}
