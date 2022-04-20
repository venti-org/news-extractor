use reqwest::Client;
use serde_json::Value;
use serde_json::Map;

pub async fn render(url: &str, api: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = Map::<String, Value>::new();
    body.insert("url".into(), Value::String(url.into()));
    body.insert("task_mask".into(), Value::Number(4.into()));
    let data = serde_json::to_string(&Value::Object(body))?;
    let resp = Client::builder()
        .build()?
        .post(api)
        .header("Content-Type", "application/json")
        .body(data).send()
        .await?
        .json::<Map<String, Value>>()
        .await?;

    let html = resp.get("render_html")
        .ok_or("not found render_html")?.as_str()
        .ok_or("not found render_html")?.to_string();
    if html.is_empty() {
        Err("html is empty".into())
    } else {
        Ok(html)
    }
}
