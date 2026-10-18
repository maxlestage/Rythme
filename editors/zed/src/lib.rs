//! The Zed extension of Rythme: starts the language server `rythme lsp`.
//!
//! The program is `rythme` on the PATH, or the one given in the settings of
//! Zed: `"lsp": {"rythme": {"binary": {"path": "~/.cargo/bin/rythme"}}}`.
//! `initialization_options` (`{"language": "fr"}`) and `settings` are passed
//! to the server.

use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

struct Rythme;

const SERVER: &str = "rythme";

impl zed::Extension for Rythme {
    fn new() -> Self {
        Rythme
    }

    fn language_server_command(&mut self, _id: &LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(SERVER, worktree).ok().and_then(|s| s.binary);
        let path = binary.as_ref().and_then(|b| b.path.clone()).or_else(|| worktree.which("rythme"));
        let Some(command) = path else {
            return Err("Rythme is not installed: `rythme` is not on the PATH (see https://github.com/maxlestage/Rythme#install)\n\
                 Rythme n'est pas installé : `rythme` n'est pas dans le PATH (voir https://github.com/maxlestage/Rythme/blob/main/README.fr.md)"
                .into());
        };
        let args = binary.as_ref().and_then(|b| b.arguments.clone()).unwrap_or_else(|| vec!["lsp".into()]);
        let mut env = worktree.shell_env();
        if let Some(extra) = binary.and_then(|b| b.env) {
            env.extend(extra);
        }
        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree).ok().and_then(|s| s.initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(SERVER, worktree).ok().and_then(|s| s.settings);
        Ok(settings.map(|s| serde_json::json!({ "rythme": s })))
    }
}

zed::register_extension!(Rythme);
