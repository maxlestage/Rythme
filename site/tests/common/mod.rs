//! Helpers shared by the tests.

/// `rel` resolved against the folder of `base` (both paths of the site).
pub fn resolve(base: &str, rel: &str) -> String {
    let mut parts: Vec<&str> = base.split('/').collect();
    parts.pop();
    for piece in rel.split('/') {
        match piece {
            ".." => {
                parts.pop();
            }
            "." | "" => {}
            piece => parts.push(piece),
        }
    }
    let path = parts.join("/");
    if rel.ends_with('/') { format!("{path}/") } else { path }
}
