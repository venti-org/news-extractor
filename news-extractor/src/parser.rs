use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    pub url: String,
    pub title: String,
    pub image: String,
    pub content: String,
    pub feature: String,
}

impl Default for Feature {
    fn default() -> Self {
        Self { 
            url: Default::default(), 
            title: Default::default(), 
            image: Default::default(),
            content: Default::default(),
            feature: Default::default(),
        }
    }
}

use regex::Regex;
use render_dom::Handle;
use serde_json::Value;


fn collapse_str(s: &str) -> String {
    let re = Regex::new(r" +").unwrap();
    re.replace_all(s, " ").to_string()
}

fn find_title(root: &Handle) -> Option<String> {
    root.borrow().title().map(|s| collapse_str(&s))
}

fn find_image(root: &Handle) -> Option<String> {
    root.borrow().ld_json().map_or(None,|s| {
        let v: Value = serde_json::from_str(&s).ok()?;
        let result = v.as_object()?;
        Some(result["image"].as_object()?["url"].as_str()?.to_string())
    })
}

fn find_content(root: &Handle) -> Option<String> {
    let body = root.borrow().body();
    let main = body.borrow().descendants().find(|child| {
        if !child.borrow().is_element_node() {
            return false;
        }
        if let Some(_class) = child.borrow().attr("class") {
            if _class.contains("article-body") {
                return true;
            }
        }
        return false;
    });
    let node = main.unwrap_or(body);
    let borrow = node.borrow();
    Some(collapse_str(&borrow.descendants_text_skip(|node| {
        if !node.is_element_node() {
            return false;
        }
        if node.is_none_tag() {
            return true;
        }
        if let Some(_class) = node.attr("class") {
            let rubbish_classes = vec!["featured-video", "speechkit-wrapper", "image-ct"];
            if rubbish_classes.iter().any(|c| _class.contains(c)) {
                return true;
            }
        }
        if node.tag_name() == "a" {
            if let Some(child) = node.first_child() {
                if child.borrow().is_element_node() && child.borrow().tag_name() == "strong" {
                    let text = child.borrow().descendants_text();
                    if !text.is_empty() && text.chars().any(|c| c.is_uppercase()){
                        return true;
                    }
                }
            }
        }
        false
    })).trim().to_string())
}

fn find_feature(root: &Handle) -> Option<String> {
    let body = root.borrow().body();
    let borrow = body.borrow();
    let vision = &borrow.custom_node_data().vision;
    Some(format!("{}:{}:{}:{}:{}", vision.width, vision.height, vision.xpos, vision.ypos, vision.visible))
}

pub(crate) fn parse_tree(url: String, root: Handle) -> Result<Feature, String>{
    let mut feature = Feature::default();
    feature.url = url;
    let root = &root;
    feature.title = find_title(root).ok_or("not found title".to_string())?;
    if let Some(image) = find_image(root) {
        feature.image = image;
    }
    feature.content = find_content(root).ok_or("not found content".to_string())?;
    feature.feature = find_feature(root).ok_or("not found feature".to_string())?;
    Ok(feature)
}