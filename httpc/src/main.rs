use clap::Parser;
use std::process::Command;

/// Zed HTTP Client — backend CLI that executes requests defined in .http files.
#[derive(Parser)]
#[command(name = "httpc", version, about)]
struct Args {
    /// HTTP method (GET, POST, PUT, DELETE, ...)
    #[arg(long)]
    method: String,

    /// Request URL
    #[arg(long)]
    url: String,

    /// Request body (empty for methods without body)
    #[arg(long, default_value = "")]
    body: String,
}

fn main() {
    let args = Args::parse();

    let status = Command::new("curl")
        .arg("-X")
        .arg(&args.method)
        .arg(&args.url)
        .arg("--data-raw")
        .arg(&args.body)
        .status()
        .expect("failed to spawn curl");

    std::process::exit(status.code().unwrap_or(1));
}
