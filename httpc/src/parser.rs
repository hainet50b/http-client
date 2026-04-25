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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_get_request() {
        let content = "GET https://example.com/api\nAccept: application/json\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.url, "https://example.com/api");
        assert_eq!(
            req.headers,
            vec![("Accept".to_string(), "application/json".to_string())]
        );
        assert_eq!(req.body, "");
    }

    #[test]
    fn parses_post_with_json_body() {
        let content =
            "POST https://example.com/users\nContent-Type: application/json\n\n{\"name\":\"alice\"}\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.url, "https://example.com/users");
        assert_eq!(
            req.headers,
            vec![(
                "Content-Type".to_string(),
                "application/json".to_string()
            )]
        );
        assert_eq!(req.body, "{\"name\":\"alice\"}");
    }

    #[test]
    fn parses_url_with_query_string() {
        let content = "GET /api?role=admin&limit=10\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.url, "/api?role=admin&limit=10");
    }

    #[test]
    fn ignores_http_version_after_url() {
        let content = "GET /api HTTP/1.1\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.url, "/api");
    }

    #[test]
    fn handles_no_body() {
        let content = "DELETE /users/42\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.method, "DELETE");
        assert_eq!(req.url, "/users/42");
        assert!(req.headers.is_empty());
        assert_eq!(req.body, "");
    }

    #[test]
    fn handles_multiline_body() {
        let content =
            "POST /a\nContent-Type: application/xml\n\n<root>\n  <child>value</child>\n</root>\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.body, "<root>\n  <child>value</child>\n</root>");
    }

    #[test]
    fn parses_multiple_headers() {
        let content =
            "GET /api\nAccept: application/json\nAuthorization: Bearer token\nX-Trace-Id: abc\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(
            req.headers,
            vec![
                ("Accept".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), "Bearer token".to_string()),
                ("X-Trace-Id".to_string(), "abc".to_string()),
            ]
        );
    }

    #[test]
    fn header_value_can_contain_colon() {
        let content = "GET /api\nContent-Type: application/json; charset=utf-8\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.headers[0].1, "application/json; charset=utf-8");
    }

    #[test]
    fn skips_comments_above_method_line() {
        let content = "# this is a comment\n// also a comment\nGET /api\n";
        let req = parse_request_at(content, 1).unwrap();
        assert_eq!(req.method, "GET");
    }

    #[test]
    fn parses_request_within_separated_blocks() {
        let content = "### first\nGET /a\n\n### second\nPOST /b\n";
        let req = parse_request_at(content, 5).unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.url, "/b");
    }

    #[test]
    fn resolves_request_when_clicked_on_body_line() {
        let content =
            "POST /users\nContent-Type: application/json\n\n{\n  \"name\": \"alice\"\n}\n";
        let req = parse_request_at(content, 5).unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.body, "{\n  \"name\": \"alice\"\n}");
    }

    #[test]
    fn resolves_request_when_clicked_on_header_line() {
        let content = "GET /api\nAccept: application/json\nX-Custom: value\n";
        let req = parse_request_at(content, 2).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.headers.len(), 2);
    }

    #[test]
    fn errors_on_out_of_range_line() {
        let content = "GET /api\n";
        let result = parse_request_at(content, 5);
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_block_without_method_line() {
        let content = "### only separator\n# comment line\n\n";
        let result = parse_request_at(content, 2);
        assert!(result.is_err());
    }
}
