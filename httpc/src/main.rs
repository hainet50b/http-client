use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod parser;

/// Zed HTTP Client — backend CLI that executes requests defined in .http files.
#[derive(Parser)]
#[command(name = "httpc", version, about)]
struct Args {
    /// Path to the .http file
    #[arg(long)]
    file: PathBuf,

    /// 1-based line number; the request containing this line is executed
    #[arg(long)]
    line: usize,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.file)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", args.file.display(), e));
    let req = parser::parse_request_at(&content, args.line)
        .unwrap_or_else(|e| panic!("parse error: {}", e));

    let mut cmd = Command::new("curl");
    cmd.arg("-X").arg(&req.method);
    cmd.arg(&req.url);
    for (name, value) in &req.headers {
        cmd.arg("-H").arg(format!("{name}: {value}"));
    }
    if !req.body.is_empty() {
        cmd.arg("--data-raw").arg(&req.body);
    }

    let status = cmd.status().expect("failed to spawn curl");
    std::process::exit(status.code().unwrap_or(1));
}
