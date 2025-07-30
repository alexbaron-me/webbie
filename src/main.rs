use clap::Parser;
use colored::Colorize;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
use webbie::{start_server, Request, RequestLogger};

#[derive(Parser, Debug)]
#[command(name = "webbie")]
#[command(about = "A simple HTTP server that logs requests", long_about = None)]
struct Args {
    #[arg(short, long, help = "Port to listen on")]
    port: u16,
}

fn format_body(mime_type: &str, body: &[u8]) -> String {
    match mime_type.to_lowercase().trim() {
        "application/json" => {
            // TODO: Move this away from the hot path
            let ss = SyntaxSet::load_defaults_newlines();
            let ts = ThemeSet::load_defaults();
            let syntax = ss.find_syntax_by_extension("json").unwrap();
            // TODO: Better theme selection
            let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

            // TODO: Use a string builder or something
            let raw = String::from_utf8_lossy(body);

            // TODO: No need to allocate a new vector here, can use a stream instead
            let mut lines = vec![];
            for line in LinesWithEndings::from(raw.as_ref()) {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                lines.push(escaped);
            }

            lines.join("\n")
        }
        _ => {
            // For other MIME types, just return the body as a string
            String::from_utf8_lossy(body).to_string()
        }
    }
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
                    k.as_str().to_lowercase().yellow().bold(),
                    v.to_str().unwrap_or("Invalid UTF-8")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let body_str = if req.body().len() > 0 {
            let mime_type = req
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("text/plain");

            let formatted_body = format_body(mime_type, req.body());
            formatted_body
        } else {
            "".into()
        };

        let query = if req.query().is_empty() {
            String::new()
        } else {
            format!("?{}", req.query().yellow().italic())
        };

        println!(
            "{} {}{}\n{}\n{}\n",
            req.method().as_str().green().bold(),
            req.path().as_str().blue().bold(),
            query,
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
