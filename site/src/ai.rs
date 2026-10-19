//! What the site gives AI assistants: `/llms.txt` and `/llms-full.txt` (the files of
//! `docs/`, the whole language in one text), the configurations of the MCP server
//! (`rythme mcp`, as the manual gives them) and the commands of the loop of
//! `--format json` and `rythme fix`, shown on `/{lang}/ai/`.

use crate::i18n::Lang;
use crate::routes::{Page, Route};

/// `docs/llms.txt`: the index, in the llms.txt format (<https://llmstxt.org>).
pub const LLMS: &str = include_str!("../../docs/llms.txt");
/// `docs/llms-full.txt`: the whole language in one text (`rythme help llms`).
pub const LLMS_FULL: &str = include_str!("../../docs/llms-full.txt");
/// Where `docs/llms.txt` points for the complete file: served by the site instead.
const LLMS_FULL_ON_GITHUB: &str = "https://github.com/maxlestage/Rythme/blob/master/docs/llms-full.txt";

/// The paths of the two files, at the root of the site (behind the base path).
pub const LLMS_PATH: &str = "/llms.txt";
pub const LLMS_FULL_PATH: &str = "/llms-full.txt";

/// The version of Rythme, from the `Cargo.toml` of the interpreter.
pub fn version() -> &'static str {
    const TOML: &str = include_str!("../../Cargo.toml");
    TOML.lines()
        .find_map(|line| line.strip_prefix("version = \"").and_then(|v| v.strip_suffix('"')))
        .expect("the version of Rythme in Cargo.toml")
}

/// Adds the MCP server to Claude Code (section 7.10 of the manual).
pub const CLAUDE_CODE: &str = "claude mcp add rythme -- rythme mcp";
/// `.mcp.json` (Claude Code), `.cursor/mcp.json` (Cursor), `claude_desktop_config.json`
/// (Claude Desktop): the same configuration (section 7.10 of the manual).
pub const MCP_JSON: &str = "{\n  \"mcpServers\": {\n    \"rythme\": { \"command\": \"rythme\", \"args\": [\"mcp\"] }\n  }\n}";
/// `.vscode/mcp.json`: GitHub Copilot in VS Code (section 7.10 of the manual).
pub const VSCODE_JSON: &str =
    "{\n  \"servers\": {\n    \"rythme\": { \"type\": \"stdio\", \"command\": \"rythme\", \"args\": [\"mcp\"] }\n  }\n}";

/// The configurations of the MCP server: (the assistants, the language of the snippet,
/// the snippet), in the order of `Texts::ai_mcp_where`.
pub const MCP: [(&str, &str, &str); 3] =
    [("Claude Code", "sh", CLAUDE_CODE), ("Claude Desktop · Cursor · .mcp.json", "json", MCP_JSON), ("VS Code", "json", VSCODE_JSON)];

/// The commands of the loop, in the order of `Texts::ai_loop`.
pub const LOOP: [&str; 5] = [
    "rythme check --format json main.ry",
    "rythme fix main.ry",
    "rythme explain E003",
    "rythme test --format json",
    "rythme run --format json main.ry",
];

/// What `rythme fix` does, from the manual (section 7.10).
pub const FIX_EXAMPLE: &str = "$ cat habits.ry\nlet x = 3\nprint(x > 2 && x < 9)\nprint(x > 2 ? \"big\" : \"small\")\n$ rythme fix habits.ry\n  ✓ Remove `let`\n  ✓ Replace with `and`\n  ✓ Replace with `if x > 2 then \"big\" else \"small\"`\nhabits.ry: 3 fix(es) applied\n$ cat habits.ry\nx = 3\nprint(x > 2 and x < 9)\nprint(if x > 2 then \"big\" else \"small\")";

/// A new project for an agent: `rythme new` writes its `AGENTS.md`.
pub const NEW_PROJECT: &str = "rythme new my-project";

/// The first message to paste into a chat, in the language of the page.
pub fn prompt(lang: Lang, origin: &str) -> String {
    lang.texts().ai_prompt.replace("{url}", &format!("{origin}{LLMS_FULL_PATH}"))
}

/// `/llms.txt`: `docs/llms.txt` with the complete file on this site, and the pages of the
/// site as Markdown (`origin`: the public address, with the base path).
pub fn llms_txt(origin: &str) -> String {
    let t = Lang::En.texts();
    let md = |page: Page| format!("{origin}{}", Route::new(Lang::En, page).markdown_path());
    let others: Vec<String> = Lang::ALL.iter().filter(|l| **l != Lang::En).map(|l| format!("`/{}/`", l.code())).collect();
    let guide = crate::teach::render(Lang::En);
    let website = [
        format!("- [{}]({}): {}", t.ai_title, md(Page::Ai), t.ai_lead),
        format!("- [{}]({}): {}", t.learn_title, md(Page::Learn), t.learn_lead),
        format!("- [{}]({}): {}", t.ex_title, md(Page::Examples), t.ex_lead),
        format!("- [Rythme]({}): {}", md(Page::Home), t.meta_description),
        format!("- [{}]({}): {}", guide.title, md(Page::Teach), t.teach_lead),
        format!(
            "- [{}]({origin}{}): {} A link `#code=` followed by a program (percent-encoded) opens it.",
            t.pg_title,
            Route::new(Lang::En, Page::Playground).path(),
            t.pg_lead
        ),
        format!(
            "- [The same pages in other languages]({origin}/fr/index.md): French, Spanish, German, Portuguese and Chinese, with {} in place of `/en/`",
            others.join(", ")
        ),
    ];
    let section = format!("## Website\n\n{}\n\n", website.join("\n"));
    let text = LLMS.replace(LLMS_FULL_ON_GITHUB, &format!("{origin}{LLMS_FULL_PATH}"));
    match text.find("## Optional") {
        Some(at) => format!("{}{section}{}", &text[..at], &text[at..]),
        None => format!("{}\n{section}", text.trim_end()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_index_points_to_the_site() {
        let text = llms_txt("https://example.com/Rythme");
        assert!(!text.contains(LLMS_FULL_ON_GITHUB), "{text}");
        assert!(text.contains("](https://example.com/Rythme/llms-full.txt)"), "{text}");
        assert!(text.contains("](https://example.com/Rythme/en/learn/index.md)"), "{text}");
        assert!(text.find("## Website").unwrap() < text.find("## Optional").unwrap());
        assert!(text.starts_with("# Rythme\n"));
    }

    #[test]
    fn the_version_is_read() {
        let v = version();
        assert!(v.split('.').count() == 3 && v.chars().all(|c| c.is_ascii_digit() || c == '.'), "{v}");
    }
}
