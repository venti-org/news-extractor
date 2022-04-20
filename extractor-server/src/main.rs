mod render;
mod parser;
mod server;

use clap::Arg;
use clap::Command;
use parser::main_parser;
use server::main_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new("extractor")
        .version("1.0").about("new extractor")
        .arg_required_else_help(true)
        .subcommand(Command::new("parse")
            .arg(Arg::new("render-server").long("--render-server").short('r')
                .default_value("http://localhost:3000/render"))
            .arg(Arg::new("stdin").takes_value(false))
            .arg(Arg::new("url").long("-url").short('u').required(true).takes_value(true))
        ).subcommand(Command::new("server")
            .arg(Arg::new("host").long("--host").default_value("0.0.0.0"))
            .arg(Arg::new("port").long("--port").short('p').default_value("8080"))
            .arg(Arg::new("concurrency").long("--concurrency").short('c').default_value("4"))
        );
    let matches = app.get_matches();
    if let Some((cmd, matches)) = matches.subcommand() {
        match cmd {
            "server" => main_server(matches).await,
            "parse" => main_parser(matches).await,
            _ => Err(format!("not found subcommand: {}", cmd).into())
        }
    } else {
        Err("use --help".into())
    }
}
