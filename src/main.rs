use clap::Parser;
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
        println!("Got a request");
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let logger = ConsoleLogger;

    start_server(args.port, logger).await;
}
