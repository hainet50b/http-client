#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub fn parse_request_at(content: &str, line: usize) -> Result<Request, String> {
    let lines: Vec<&str> = content.lines().collect();

    if line < 1 || line > lines.len() {
        return Err(format!(
            "line {} is out of range (file has {} lines)",
            line,
            lines.len()
        ));
    }

    let target_idx = line - 1;

    let mut block_start = 0;
    for i in (0..=target_idx).rev() {
        if lines[i].starts_with("###") {
            block_start = i + 1;
            break;
        }
    }

    let mut block_end = lines.len();
    for i in (target_idx + 1)..lines.len() {
        if lines[i].starts_with("###") {
            block_end = i;
            break;
        }
    }

    let block = &lines[block_start..block_end];

    let mut i = 0;
    while i < block.len() {
        let trimmed = block[i].trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            i += 1;
        } else {
            break;
        }
    }

    if i >= block.len() {
        return Err("no method line found in request block".to_string());
    }

    let method_line = block[i];
    let mut parts = method_line.split_whitespace();
    let method = parts.next().ok_or("missing method")?.to_string();
    let url = parts.next().ok_or("missing URL")?.to_string();
    i += 1;

    let mut headers = Vec::new();
    while i < block.len() {
        let header_line = block[i];
        if header_line.trim().is_empty() {
            break;
        }
        if let Some(colon_pos) = header_line.find(':') {
            let name = header_line[..colon_pos].trim().to_string();
            let value = header_line[colon_pos + 1..].trim().to_string();
            headers.push((name, value));
        }
        i += 1;
    }

    if i < block.len() {
        i += 1;
    }

    let body = block[i..].join("\n").trim().to_string();

    Ok(Request {
        method,
        url,
        headers,
        body,
    })
}
