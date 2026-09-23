pub mod packages;

use std::process::Command;

use anyhow::{Context, Result};

use crate::catalog::ScriptEntry;

pub trait Module {
    fn name(&self) -> &str;
    fn supports(&self, distro: &str) -> bool;
    fn run(&self, script: &ScriptEntry) -> Result<()>;
}

pub fn execute(script: &ScriptEntry) -> Result<()> {
    if let Some(description) = &script.annotation.description {
        println!("> {}", description);
    }

    let use_sudo = script.annotation.requires.iter().any(|r| r == "sudo");

    let mut cmd = Command::new("bash");
    if use_sudo {
        cmd.arg("-c");
        cmd.arg(format!("sudo bash {}", script.path.display()));
    } else {
        cmd.arg(&script.path);
    }

    let status = cmd
        .status()
        .with_context(|| format!("failed to execute {}", script.name))?;

    let code = status.code().unwrap_or(-1);
    if code == 0 || script.annotation.exit_codes.contains(&code) {
        return Ok(());
    }

    anyhow::bail!("script {} failed with exit code {}", script.name, code)
}
