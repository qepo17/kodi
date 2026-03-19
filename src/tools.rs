use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Error, Ok};

pub fn execute_tool(name: &str, arguments: &str) -> Result<String, Error> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();
    let workdir = std::env::current_dir()?;

    match name {
        "bash" => {
            let cmd = args["command"].as_str().unwrap_or("echo 'no command'");
            run_bash(&workdir, cmd)
        }
        "read_file" => {
            let path = args["path"].as_str().context("missing_path")?;
            let limit = args
                .get("limit")
                .and_then(|v| v.as_u64())
                .map(|n| n as usize);
            run_read(&workdir, path, limit)
        }
        _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
    }
}

fn safe_path(workdir: &Path, p: &str) -> Result<PathBuf, Error> {
    let path = workdir
        .join(p)
        .canonicalize()
        .with_context(|| format!("Invalid path {}", p))?;
    if !path.starts_with(workdir) {
        return Err(anyhow::anyhow!("Path escapes workspace: {p}"));
    }
    Ok(path)
}

fn run_read(workdir: &Path, path: &str, limit: Option<usize>) -> Result<String, Error> {
    let path = safe_path(workdir, path)?;
    let text =
        fs::read_to_string(&path).with_context(|| format!("Failed to read: {}", path.display()))?;

    let result: String = match limit {
        Some(n) => text.lines().take(n).collect::<Vec<_>>().join("\n"),
        None => text,
    };

    // Truncate to 50000 chars
    if result.len() > 50_000 {
        Ok(format!(
            "{}... [truncated]",
            result.chars().take(50_000).collect::<String>()
        ))
    } else {
        Ok(result)
    }
}

fn run_bash(workdir: &Path, cmd: &str) -> Result<String, Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .current_dir(workdir)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(format!("{}{}", stdout, stderr))
}
