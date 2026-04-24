use std::process::Command;

fn main() {
    let mut method = String::new();
    let mut url = String::new();
    let mut body = String::new();

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--method" => method = args.next().unwrap_or_default(),
            "--url" => url = args.next().unwrap_or_default(),
            "--body" => body = args.next().unwrap_or_default(),
            _ => {}
        }
    }

    let status = Command::new("curl")
        .arg("-X")
        .arg(&method)
        .arg(&url)
        .arg("--data-raw")
        .arg(&body)
        .status()
        .expect("failed to spawn curl");

    std::process::exit(status.code().unwrap_or(1));
}
