use std::path::PathBuf;
use zed_extension_api::{self as zed, serde_json::Value, settings::LspSettings};

struct BasedPyright;

impl zed::Extension for BasedPyright {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let env = worktree.shell_env();

        if let Ok(lsp_settings) = LspSettings::for_worktree("basedpyright-alt", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    let args = binary.arguments.unwrap_or(vec!["--stdio".to_string()]);
                    return Ok(zed::Command {
                        command: path,
                        args,
                        env,
                    });
                }
            }
        }

        let path = worktree
            .which("basedpyright-langserver")
            .ok_or_else(|| "basedpyright must be installed and available in $PATH.".to_string())?;
        Ok(zed::Command {
            command: path,
            args: vec!["--stdio".to_string(), Default::default()],
            env,
        })
    }
    // ref https://github.com/zed-industries/zed/blob/main/extensions/ruff/src/ruff.rs
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let mut settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        settings = update_python_path(settings, worktree.root_path())?;
        Ok(Some(settings))
    }
}

fn update_python_path(mut settings: Value, root: String) -> zed_extension_api::Result<Value> {
    if let Some(python) = settings.get_mut("python") {
        if let Some(python_path) = python.get_mut("pythonPath") {
            let ppath_str = python_path
                .as_str()
                .ok_or_else(|| "Could not parse pythonPath to a valid string. Please ensure pythonPath is a string".to_string())?;
            let venv_path = PathBuf::from_iter([&root, ppath_str]);

            *python_path = venv_path
                .to_str()
                .ok_or_else(|| format!("Could not parse path {venv_path:?} for pythonPath"))?
                .into();
        }
    }

    Ok(settings)
}

zed::register_extension!(BasedPyright);
