//! HTTP requests: `http_get(url, headers)` and `http_post(url, body,
//! headers)`, to fetch a page or call a web service (a language model, for
//! example). Like `read_file`, they give `Ok(text)` or `Err(message)`: a
//! request that fails (no network, a wrong address, a server that answers
//! 404, a timeout) is a value to handle, never a crash.
//!
//! The request is made by the program `curl`, which comes with Windows 10
//! and later, macOS and most Linux systems, as `git` does the downloads of
//! `rythme install`: Rythme keeps no dependency for HTTPS, and curl brings
//! the system's certificates, proxies and TLS. The address, the headers and
//! the body are given to curl on its standard input (`--config -`), never
//! on its command line, so that an API key in a header does not show in the
//! list of processes; `~/.curlrc` is ignored (`--disable`). Only `http://`
//! and `https://` are allowed, redirections included; a request takes at
//! most 30 seconds (10 to connect). The playground runs in a browser, which
//! has no curl: there, both give an `Err` that says so.

use std::io::Write as _;
use std::process::{Command, Stdio};

use super::*;

/// The longest part of the answer of a server shown in an `Err`, in characters.
const SHOWN: usize = 2000;

pub(super) fn http_get(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let url = text(a, 0, "http_get", span)?;
    let headers = if a.len() > 1 { headers(a, 1, "http_get", span)? } else { Vec::new() };
    Ok(request(it, "http_get", url, None, &headers))
}

pub(super) fn http_post(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let url = text(a, 0, "http_post", span)?;
    let body = text(a, 1, "http_post", span)?;
    let headers = if a.len() > 2 { headers(a, 2, "http_post", span)? } else { Vec::new() };
    Ok(request(it, "http_post", url, Some(body), &headers))
}

/// The headers given as argument `i`: a record or a dict of texts.
fn headers(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Vec<(String, String)>> {
    let pairs: Vec<(String, Value)> = match &a[i] {
        Value::Record(r) => r.iter().map(|(k, v)| (k.to_string(), v.clone())).collect(),
        Value::Dict(m) => {
            let mut out = Vec::new();
            for (k, v) in m.iter() {
                let Value::Text(k) = k else {
                    return Err(bad_header(f, &k.describe(), span));
                };
                out.push((k.to_string(), v.clone()));
            }
            out
        }
        v => return Err(wrong(f, i, Expected::RecordOrDict, v, span)),
    };
    let mut out = Vec::new();
    for (name, value) in pairs {
        let value = match value {
            Value::Text(t) => t.to_string(),
            v @ (Value::Int(_) | Value::Big(_) | Value::Float(_)) => v.display(),
            v => return Err(bad_header(f, &v.describe(), span)),
        };
        out.push((name, value));
    }
    Ok(out)
}

fn bad_header(f: &str, got: &str, span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "E005",
        t!(
            "`{f}`: the names and the values of the headers are texts, not {got}",
            "`{f}` : les noms et les valeurs des en-têtes sont des textes, pas {got}"
        ),
    )
    .at(span)
    .help(t!(
        "for example: {{\"content-type\": \"application/json\", \"x-api-key\": key}}",
        "par exemple : {{\"content-type\": \"application/json\", \"x-api-key\": cle}}"
    ))
    .boxed()
}

/// A text written between quotes in a configuration file of curl.
fn quoted(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The part of a URL that names the server: `api.example.com`.
fn host(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..end];
    authority.rsplit_once('@').map_or(authority, |(_, h)| h)
}

fn request(it: &mut Interp, f: &str, url: &str, body: Option<&str>, headers: &[(String, String)]) -> Value {
    let err = |it: &mut Interp, msg: String| it.err(text_value(msg));
    if cfg!(target_arch = "wasm32") {
        return err(
            it,
            t!(
                "`{f}` is not available in the browser: the playground cannot make requests. Run the program on your computer: rythme file.ry",
                "`{f}` n'est pas disponible dans le navigateur : le playground ne peut pas faire de requêtes. Lancez le programme sur votre ordinateur : rythme fichier.ry"
            ),
        );
    }
    let lower = url.to_ascii_lowercase();
    if !(lower.starts_with("http://") || lower.starts_with("https://")) || host(url).is_empty() {
        return err(
            it,
            t!(
                "a web address starts with http:// or https://, not \"{url}\"",
                "une adresse web commence par http:// ou https://, pas \"{url}\""
            ),
        );
    }
    if url.chars().any(char::is_control)
        || headers.iter().any(|(k, v)| k.is_empty() || k.contains([':', ' ']) || (k.clone() + v).chars().any(char::is_control))
    {
        return err(
            it,
            t!(
                "an address or a header cannot hold a line break or a control character, and a header name is a word without `:` or space",
                "une adresse ou un en-tête ne peut pas contenir de retour à la ligne ni de caractère de contrôle, et un nom d'en-tête est un mot sans `:` ni espace"
            ),
        );
    }
    let mut config = format!("url = {}\n", quoted(url));
    for (name, value) in headers {
        config.push_str(&format!("header = {}\n", quoted(&format!("{name}: {value}"))));
    }
    if let Some(body) = body {
        config.push_str(&format!("data-binary = {}\n", quoted(body)));
    }
    match run_curl(&config) {
        Ok((code, answer)) if (200..300).contains(&code) => it.ok(text_value(answer)),
        Ok((code, answer)) => {
            let shown: String = answer.chars().take(SHOWN).collect();
            let more = if shown.len() < answer.len() { "..." } else { "" };
            err(it, t!("the server answered {code}: {shown}{more}", "le serveur a répondu {code} : {shown}{more}"))
        }
        Err(Failure::NoCurl) => err(
            it,
            t!(
                "`{f}` needs the program `curl`, which comes with Windows 10, macOS and most Linux systems: install it (on Debian or Ubuntu: sudo apt install curl)",
                "`{f}` a besoin du programme `curl`, fourni avec Windows 10, macOS et la plupart des systèmes Linux : installez-le (sur Debian ou Ubuntu : sudo apt install curl)"
            ),
        ),
        Err(Failure::Curl(code, said)) => {
            let host = host(url);
            let msg = match code {
                6 => t!(
                    "server not found: {host} (no connection to the Internet, or a mistake in the address?)",
                    "serveur introuvable : {host} (pas de connexion à Internet, ou une erreur dans l'adresse ?)"
                ),
                7 => t!("could not connect to {host}: {said}", "impossible de se connecter à {host} : {said}"),
                28 => t!("no answer from {host} in time (30 seconds)", "pas de réponse de {host} à temps (30 secondes)"),
                35 | 51 | 53 | 54 | 58 | 59 | 60 | 66 | 77 | 80 | 82 | 83 | 90 | 91 => {
                    t!("the secure connection (TLS) to {host} failed: {said}", "la connexion sécurisée (TLS) à {host} a échoué : {said}")
                }
                _ => t!("the request to {host} failed: {said}", "la requête à {host} a échoué : {said}"),
            };
            err(it, msg)
        }
    }
}

enum Failure {
    /// curl is not installed.
    NoCurl,
    /// curl stopped with this exit code and this message.
    Curl(i32, String),
}

/// Runs curl with this configuration: the status code of the answer and its body.
fn run_curl(config: &str) -> Result<(u16, String), Failure> {
    let child = Command::new("curl")
        .args([
            "--disable",
            "--silent",
            "--show-error",
            "--location",
            "--max-redirs",
            "5",
            "--proto",
            "=http,https",
            "--proto-redir",
            "=http,https",
            "--connect-timeout",
            "10",
            "--max-time",
            "30",
            "--write-out",
            "\n%{http_code}",
            "--config",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(Failure::NoCurl),
        Err(e) => return Err(Failure::Curl(-1, e.to_string())),
    };
    if let Some(mut stdin) = child.stdin.take() {
        // A curl that stops early closes its input: its exit code says why.
        let _ = stdin.write_all(config.as_bytes());
    }
    let out = child.wait_with_output().map_err(|e| Failure::Curl(-1, e.to_string()))?;
    let said = String::from_utf8_lossy(&out.stderr).trim().trim_start_matches("curl: ").to_string();
    let stdout = String::from_utf8_lossy(&out.stdout);
    if !out.status.success() {
        return Err(Failure::Curl(out.status.code().unwrap_or(-1), said));
    }
    let (body, code) = stdout.rsplit_once('\n').unwrap_or(("", &stdout));
    match code.trim().parse::<u16>() {
        Ok(code) => Ok((code, body.to_string())),
        Err(_) => Err(Failure::Curl(-1, said)),
    }
}
