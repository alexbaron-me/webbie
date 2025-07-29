use clap::Parser;
use colored::Colorize;
use webbie::{start_server, Request, RequestLogger};

#[derive(Parser, Debug)]
#[command(name = "webbie")]
#[command(about = "A simple HTTP server that logs requests", long_about = None)]
struct Args {
    #[arg(short, long, help = "Port to listen on")]
    port: u16,
}

struct ConsoleLogger;
impl RequestLogger for ConsoleLogger {
    fn log_request(&self, req: &Request) {
        let headers = req
            .headers()
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}: {}",
                    k.as_str().to_lowercase().bold(),
                    v.to_str().unwrap_or("Invalid UTF-8")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let body_str = String::from_utf8_lossy(req.body().as_ref());

        println!(
            "{} {}\n{}\n{}\n",
            req.method().as_str().green().bold(),
            req.path().as_str().blue().bold(),
            headers,
            body_str
        );
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let logger = ConsoleLogger;

    start_server(args.port, logger).await;
}
