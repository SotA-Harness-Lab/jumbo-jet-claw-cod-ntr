//! Clipboard forwarding to one attached client in the current pane's tmux session.
//!
//! Capability inspection and delivery use the same client and trusted system
//! executable. The parent routing layer enforces the terminal payload limit.

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

/// Copy text through tmux's native clipboard integration.
///
/// `load-buffer -w -` lets tmux read the text from stdin, keep a matching tmux
/// paste buffer, and forward the contents to the outer terminal clipboard when
/// possible without relying on DCS passthrough.
pub(super) fn copy(text: &str) -> Result<(), String> {
    let executable = codex_utils_path::system_executable("tmux")
        .ok_or_else(|| crate::i18n::tr!("clipboard-tmux-unavailable").to_string())?;
    let path = codex_utils_path::system_path().map_err(|error| {
        crate::i18n::tr!(
            "clipboard-tmux-resolve-path-failed",
            error = error.to_string()
        )
    })?;
    let pane = std::env::var("TMUX_PANE")
        .map_err(|_| crate::i18n::tr!("clipboard-tmux-no-pane").to_string())?;
    let client = clipboard_target(
        || command_output(&executable, &path, ["show-options", "-gv", "set-clipboard"]),
        || {
            let session = command_output(
                &executable,
                &path,
                ["display-message", "-p", "-t", &pane, "#{session_id}"],
            )?;
            command_output(
                &executable,
                &path,
                [
                    "list-clients",
                    "-t",
                    session.trim(),
                    "-F",
                    "#{client_activity} #{client_name}",
                ],
            )
        },
        |client| command_output(&executable, &path, ["show-messages", "-T", "-t", client]),
    )?;

    let mut child = std::process::Command::new(&executable)
        .env("PATH", &path)
        .args(["load-buffer", "-w", "-t", &client, "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| crate::i18n::tr!("clipboard-tmux-spawn-failed", error = e.to_string()))?;

    let Some(mut stdin) = child.stdin.take() else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(crate::i18n::tr!("clipboard-tmux-open-stdin-failed").to_string());
    };

    if let Err(err) = stdin.write_all(text.as_bytes()) {
        let _ = child.kill();
        let _ = child.wait();
        return Err(crate::i18n::tr!(
            "clipboard-tmux-write-failed",
            error = err.to_string()
        ));
    }

    drop(stdin);

    let output = child
        .wait_with_output()
        .map_err(|e| crate::i18n::tr!("clipboard-tmux-wait-failed", error = e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            let status = output.status;
            Err(crate::i18n::tr!(
                "clipboard-tmux-exit-status",
                status = status.to_string()
            ))
        } else {
            Err(crate::i18n::tr!("clipboard-tmux-failed", stderr = stderr))
        }
    }
}

/// Resolve the most recently active client in this session and check its capability.
fn clipboard_target(
    set_clipboard_fn: impl FnOnce() -> Result<String, String>,
    list_clients_fn: impl FnOnce() -> Result<String, String>,
    tmux_info_fn: impl FnOnce(&str) -> Result<String, String>,
) -> Result<String, String> {
    let set_clipboard = set_clipboard_fn()?;
    if set_clipboard.trim() == "off" {
        return Err(crate::i18n::tr!("clipboard-tmux-disabled").to_string());
    }

    // list-clients is scoped to the current pane's session. Unlike tmux's
    // implicit best-client lookup, an empty list cannot select another session.
    let clients = list_clients_fn()?;
    let client = clients
        .lines()
        .filter_map(|line| {
            let (activity, client) = line.split_once(' ')?;
            Some((activity.parse::<u64>().ok()?, client))
        })
        .filter(|(_, client)| !client.is_empty())
        .max_by_key(|(activity, _)| *activity)
        .map(|(_, client)| client.to_string())
        .ok_or_else(|| crate::i18n::tr!("clipboard-tmux-no-client").to_string())?;
    let tmux_info = tmux_info_fn(&client)?;
    if !tmux_info.lines().any(|line| {
        line.split_once("Ms: (string) ")
            .is_some_and(|(_, sequence)| !sequence.trim().is_empty())
    }) {
        return Err(crate::i18n::tr!("clipboard-tmux-missing-ms").to_string());
    }

    Ok(client)
}

fn command_output<const N: usize>(
    executable: &Path,
    path: &OsStr,
    args: [&str; N],
) -> Result<String, String> {
    let output = std::process::Command::new(executable)
        .env("PATH", path)
        .args(args)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|e| crate::i18n::tr!("clipboard-tmux-spawn-failed", error = e.to_string()))?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|e| crate::i18n::tr!("clipboard-tmux-output-not-utf8", error = e.to_string()))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            let status = output.status;
            Err(crate::i18n::tr!(
                "clipboard-tmux-exit-status",
                status = status.to_string()
            ))
        } else {
            Err(crate::i18n::tr!("clipboard-tmux-failed", stderr = stderr))
        }
    }
}

#[cfg(test)]
#[path = "tmux_tests.rs"]
mod tests;
