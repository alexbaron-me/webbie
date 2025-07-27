use clap::Parser;
use webbie::start_server;

#[derive(Parser, Debug)]
#[command(name = "webbie")]
#[command(about = "A simple HTTP server that logs requests", long_about = None)]
struct Args {
    #[arg(short, long, help = "Port to listen on")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    start_server(args.port).await;
}
