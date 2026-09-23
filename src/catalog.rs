use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::annotation::{self, Annotation};

#[derive(Debug)]
pub struct ScriptEntry {
    pub name: String,
    pub path: PathBuf,
    pub annotation: Annotation,
}

pub fn discover(dir: &Path) -> Result<Vec<ScriptEntry>> {
    let mut entries = Vec::new();

    for entry in
        fs::read_dir(dir).with_context(|| format!("reading directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "sh") {
            let content =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            let annotation = annotation::parse(&content)?;
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();

            entries.push(ScriptEntry {
                name,
                path,
                annotation,
            });
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}
