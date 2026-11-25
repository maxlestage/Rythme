//! `http_get` and `http_post`, against a small server of this test on
//! 127.0.0.1: what curl sends (method, headers, body) and how answers and
//! failures come back (`Ok(text)`, `Err(message)`). Skipped, with a note,
//! where curl is not installed.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

use common::fails;

fn curl_installed() -> bool {
    let found = Command::new("curl").arg("--version").output().is_ok_and(|o| o.status.success());
    if !found {
        eprintln!("curl is not installed: the requests are not tested");
    }
    found
}

/// A request as the server received it.
struct Received {
    head: String,
    body: String,
}

/// A server that answers each of `answers` (status line and body) to one
/// request, in order, and reports what it received.
fn serve(answers: Vec<(&'static str, &'static str)>) -> (String, mpsc::Receiver<Received>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for (status, body) in answers {
            let Ok((stream, _)) = listener.accept() else { return };
            let mut reader = BufReader::new(stream);
            let mut head = String::new();
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 || line == "\r\n" {
                    break;
                }
                head.push_str(&line);
            }
            let length = head
                .lines()
                .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|n| n.trim().parse::<usize>().unwrap()))
                .unwrap_or(0);
            let mut received = vec![0; length];
            reader.read_exact(&mut received).unwrap();
            let _ = tx.send(Received { head, body: String::from_utf8(received).unwrap() });
            let mut stream = reader.into_inner();
            let answer = format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
            stream.write_all(answer.as_bytes()).unwrap();
        }
    });
    (address, rx)
}

/// Runs code, without the proxies of the machine for 127.0.0.1.
fn run(code: &str) -> (String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_rythme"))
        .args(["-e", code])
        .env("RYTHME_LANG", "en")
        .env("NO_COLOR", "1")
        .env("NO_PROXY", "127.0.0.1")
        .env("no_proxy", "127.0.0.1")
        .output()
        .unwrap();
    (String::from_utf8_lossy(&out.stdout).into_owned(), String::from_utf8_lossy(&out.stderr).into_owned())
}

#[test]
fn get_and_post() {
    if !curl_installed() {
        return;
    }
    let (url, received) = serve(vec![("200 OK", "hello"), ("201 Created", "{\"id\": 7, \"text\": \"héllo\"}")]);
    let code = format!(
        "print(http_get(\"{url}/page?q=1\"))\n\
         answer = http_post(\"{url}/items\", to_json({{name: \"Ada\", note: \"a \\\"quote\\\"\\nand a line\"}}), {{\"x-api-key\": \"secret\", \"content-type\": \"application/json\"}})\n\
         print(answer |> and_then(parse_json) |> map(fn(r) = r.text))"
    );
    let (out, err) = run(&code);
    assert_eq!(out, "Ok(\"hello\")\nOk(\"héllo\")\n", "{err}");
    let get = received.recv().unwrap();
    assert!(get.head.starts_with("GET /page?q=1 HTTP/1.1\r\n"), "{}", get.head);
    let post = received.recv().unwrap();
    assert!(post.head.starts_with("POST /items HTTP/1.1\r\n"), "{}", post.head);
    assert!(post.head.contains("x-api-key: secret\r\n") && post.head.contains("content-type: application/json\r\n"), "{}", post.head);
    assert_eq!(post.body, "{\"name\": \"Ada\", \"note\": \"a \\\"quote\\\"\\nand a line\"}");
}

#[test]
fn errors_are_values() {
    if !curl_installed() {
        return;
    }
    let (url, _received) = serve(vec![("404 Not Found", "no such page"), ("500 Internal Server Error", "")]);
    let (out, err) = run(&format!("print(http_get(\"{url}/missing\"))\nprint(is_err(http_post(\"{url}/x\", \"\")))"));
    assert_eq!(out, "Err(\"the server answered 404: no such page\")\ntrue\n", "{err}");
    // Nobody listens there (the server above is gone after two requests).
    let (out, err) = run(&format!("print(http_get(\"{url}/again\"))"));
    assert!(out.starts_with("Err(\"could not connect to 127.0.0.1:"), "{out}{err}");
}

#[test]
fn only_web_addresses_and_clean_headers() {
    let (out, _) = run("print(http_get(\"file:///etc/passwd\"), http_get(\"ftp://x\"), http_get(\"\"), http_get(\"https://\"))");
    assert_eq!(out.matches("Err(\"a web address starts with http:// or https://").count(), 4, "{out}");
    let (out, _) = run("print(http_get(\"https://a.example\\n-o x\"), http_get(\"https://a.example\", {\"x\": \"1\\r\\nevil: 2\"}), http_get(\"https://a.example\", {\"a b\": \"1\"}))");
    assert_eq!(out.matches("Err(\"an address or a header cannot hold").count(), 3, "{out}");
    fails("http_get(\"https://a.example\", {x: [1]})", "E005");
    fails("http_get(\"https://a.example\", dict([[1, \"a\"]]))", "E005");
    fails("http_get(42)", "E005");
    fails("http_post(\"https://a.example\", {n: 1})", "E005");
}
