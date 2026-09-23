use anyhow::Result;

use crate::catalog::ScriptEntry;

use super::{Module, execute};

pub struct PackagesModule;

impl PackagesModule {
    const SUPPORTED: &'static [&'static str] = &["fedora", "debian", "ubuntu", "arch"];
}

impl Module for PackagesModule {
    fn name(&self) -> &str {
        "packages"
    }

    fn supports(&self, distro: &str) -> bool {
        Self::SUPPORTED.contains(&distro)
    }

    fn run(&self, script: &ScriptEntry) -> Result<()> {
        execute(script)
    }
}
