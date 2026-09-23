mod annotation;
mod catalog;
mod modules;

use std::env;
use std::path::Path;

use anyhow::{Result, bail};

use catalog::discover;
use modules::Module;
use modules::packages::PackagesModule;

const SCRIPTS_DIR: &str = "scripts";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("list") => list(),
        Some("run") => match args.get(2) {
            Some(script) => run(script),
            None => {
                bail!("usage: infinity-setup run <script>");
            }
        },
        Some("help") | None => {
            println!("usage: infinity-setup <list | run <script>>");
            Ok(())
        }
        Some(other) => {
            bail!("unknown command: {other}");
        }
    }
}

fn list() -> Result<()> {
    let entries = discover(Path::new(SCRIPTS_DIR))?;

    if entries.is_empty() {
        println!("no scripts found in {SCRIPTS_DIR}/");
        return Ok(());
    }

    for entry in entries {
        let description = entry
            .annotation
            .description
            .as_deref()
            .unwrap_or("<no description>");
        let distro = entry.annotation.distro.as_deref().unwrap_or("<any>");
        println!("{:<24} {:<12} {}", entry.name, distro, description);
    }

    Ok(())
}

fn run(script_name: &str) -> Result<()> {
    let entries = discover(Path::new(SCRIPTS_DIR))?;

    let script = entries
        .iter()
        .find(|e| e.name == script_name)
        .ok_or_else(|| anyhow::anyhow!("script not found: {script_name}"))?;

    let module_name = script.annotation.module.as_deref().unwrap_or("");

    let module = match module_name {
        "packages" => Box::new(PackagesModule) as Box<dyn Module>,
        other => bail!("unknown module: {other}"),
    };

    if let Some(distro) = &script.annotation.distro
        && !module.supports(distro)
    {
        bail!(
            "module '{}' does not support distro '{}'",
            module.name(),
            distro
        );
    }

    module.run(script)
}
