use clap::Parser;
use colored::Colorize;
use lazy_static::lazy_static;
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

lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

fn format_body(mime_type: &str, body: &[u8]) -> String {
    let normalized_mime_type = mime_type.split(';').next().unwrap_or("").to_lowercase();

    match normalized_mime_type.trim() {
        "application/json" => {
            let syntax = SYNTAX_SET.find_syntax_by_extension("json").unwrap();
            // TODO: Better theme selection
            let mut h = HighlightLines::new(syntax, &THEME_SET.themes["base16-ocean.dark"]);

            // TODO: Use a string builder or something
            let raw = String::from_utf8_lossy(body);

            // TODO: No need to allocate a new vector here, can use a stream instead
            let mut lines = vec![];
            for line in LinesWithEndings::from(raw.as_ref()) {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &SYNTAX_SET).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                lines.push(escaped);
            }

            lines.join("\n")
        }
        "application/x-www-form-urlencoded" => {
            // For URL-encoded forms, decode and format the body
            let decoded = String::from_utf8_lossy(body);

            decoded
                .split('&')
                .map(|pair| {
                    let mut parts = pair.splitn(2, '=');
                    let key = parts.next().unwrap_or("").to_string();
                    let value = parts.next().unwrap_or("").to_string();
                    format!("{}={}", key.bold().blue(), value)
                })
                .collect::<Vec<_>>()
                .join("\n")
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

            format!(
                "{}\n{}",
                "------ Body: ------".bold().italic().dimmed(),
                formatted_body
            )
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
