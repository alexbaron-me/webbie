# webbie

A lightweight HTTP server for testing webhooks and inspecting HTTP requests with beautiful syntax highlighting.

## Features

- 🚀 Instant local HTTP server setup
- 🎨 Syntax highlighting for JSON, XML, and URL-encoded request bodies
- 📝 Detailed request logging with colored output
- 🔍 Displays HTTP method, path, query parameters, headers, and body
- ⚡ Built with Rust for blazing-fast performance

## Installation

Install webbie globally using Cargo:

```bash
cargo install webbie
```

## Usage

Start a server on any port:

```bash
webbie --port 8080
```

Or use the short flag:

```bash
webbie -p 3000
```

Once running, webbie will display all incoming HTTP requests with:
- Request method (GET, POST, etc.) in green
- Request path in blue
- Query parameters in yellow
- Headers with syntax highlighting
- Request body with automatic formatting based on Content-Type

## Example Output

```
POST /webhook/github
content-type: application/json
user-agent: GitHub-Hookshot/abc123

------ Body: ------
{
  "action": "opened",
  "number": 42,
  "pull_request": {
    "title": "Add new feature",
    "user": {
      "login": "octocat"
    }
  }
}
```

## Use Cases

- Testing webhook integrations locally
- Debugging HTTP requests from third-party services
- Inspecting API calls during development
- Learning about HTTP request structure

## License

MIT
