use std::net::SocketAddr;
use std::sync::Arc;

use axum::{ routing::{get, post}, response::IntoResponse, Json, Router, Extension };
use clap::ArgMatches;
use news_extractor::Feature;
use news_extractor::parse_html;
use tokio::sync::Semaphore;
use serde::{Serialize, Deserialize};

use crate::render::render;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Response<R> {
    code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<R>,
    msg: String,
}

impl <R> Response<R> {
    fn new<E>(result: Result<R, E>) -> Response<R>
        where E: ToString {
        match result {
            Ok(result) => {
                Response {
                    code: 0,
                    result: Some(result),
                    msg: "success".to_string(),
                }
            },
            Err(err) => {
                Response {
                    code: 0,
                    result: None,
                    msg: err.to_string(),
                }
            },
        }
    }
}

#[derive(Deserialize)]
struct Request {
    url: String,
    api: Option<String>,
    render_html: Option<String>,
}

async fn parse_handle(request: Request) -> Result<Feature, String> {
    if request.url.is_empty() {
        return Err("not exists url".into());
    }
    let render_html = 
    if request.api.is_none() {
        request.render_html.ok_or("not exists render_html".to_string())?
    } else {
        render(&request.url, &request.api.unwrap()).await.map_err(|e|e.to_string())?
    };
    tokio::task::spawn_blocking(move || {
        parse_html(request.url.clone(), &mut render_html.as_bytes()).map_err(|e| e.into())
    }).await.unwrap()
}

async fn hello_world() -> &'static str {
    return "hello world";
}

async fn parse(Extension(sem): Extension<Arc<Semaphore>>, Json(request): Json<Request>) -> impl IntoResponse {
    let _sem = sem.acquire().await.unwrap();
    Json(Response::new(parse_handle(request).await))
}

pub async fn main_server(arg: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let concurrency = arg.value_of("concurrency").unwrap().parse::<u32>()?;
    assert!(concurrency > 0);
    assert!(concurrency <= 64);

    let sem = Arc::new(Semaphore::new(concurrency as usize));

    let host = arg.value_of("host").unwrap();
    let port = arg.value_of("port").unwrap();
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/parse", post(parse)).layer(Extension(sem));

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
