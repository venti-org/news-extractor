use clap::ArgMatches;
use news_extractor::parse_html;

use crate::render::render;

pub async fn main_parser(arg: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let url = arg.value_of("url").unwrap().to_string();
    let api = arg.value_of("render-server").unwrap();
    let feature = if arg.is_present("stdin") {
        let mut stdin = std::io::stdin();
        parse_html(url, &mut stdin)?
    } else {
        let html = render(&url, api).await?;
        parse_html(url, &mut html.as_bytes())?
    };
    println!("url: {}", feature.url);
    println!("title: {}", feature.title);
    println!("image: {}", feature.image);
    println!("content: {}", feature.content);
    println!("feature: {}", feature.feature);
    Ok(())
}
