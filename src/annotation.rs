use anyhow::{Context, Result};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Annotation {
    pub module: Option<String>,
    pub description: Option<String>,
    pub distro: Option<String>,
    pub requires: Vec<String>,
    pub exit_codes: Vec<i32>,
}

pub fn parse(content: &str) -> Result<Annotation> {
    let mut annotation = Annotation::default();

    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("# @") {
            continue;
        }

        let directive = trimmed
            .trim_start_matches('#')
            .trim()
            .trim_start_matches('@');
        let (key, value) = directive
            .split_once(':')
            .context("annotation directive must be in the format '@key: value'")?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "module" => annotation.module = Some(value.to_string()),
            "description" => annotation.description = Some(value.to_string()),
            "distro" => annotation.distro = Some(value.to_string()),
            "requires" => {
                if !value.is_empty() {
                    annotation.requires.push(value.to_string());
                }
            }
            "exit-codes" => {
                for code in value.split_whitespace() {
                    if let Ok(code) = code.parse::<i32>() {
                        annotation.exit_codes.push(code);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(annotation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_keys() {
        let content = "\
#!/usr/bin/env bash
# @module: packages
# @description: Installs base packages
# @distro: fedora
# @requires: sudo
";
        let annotation = parse(content).unwrap();
        assert_eq!(annotation.module.as_deref(), Some("packages"));
        assert_eq!(
            annotation.description.as_deref(),
            Some("Installs base packages")
        );
        assert_eq!(annotation.distro.as_deref(), Some("fedora"));
        assert_eq!(annotation.requires, vec!["sudo"]);
    }

    #[test]
    fn ignores_non_annotation_lines() {
        let content = "\
#!/usr/bin/env bash
sudo dnf check-upgrade
# this is not an annotation
";
        let annotation = parse(content).unwrap();
        assert_eq!(annotation.module, None);
        assert_eq!(annotation.requires, Vec::<String>::new());
    }

    #[test]
    fn collects_multiple_requires() {
        let content = "\
# @requires: sudo
# @requires: network
";
        let annotation = parse(content).unwrap();
        assert_eq!(annotation.requires, vec!["sudo", "network"]);
    }

    #[test]
    fn ignores_unknown_keys() {
        let content = "# @unknown: value\n";
        let annotation = parse(content).unwrap();
        assert_eq!(annotation, Annotation::default());
    }

    #[test]
    fn errors_on_malformed_directive() {
        let content = "# @missing-colon\n";
        assert!(parse(content).is_err());
    }

    #[test]
    fn parses_exit_codes() {
        let content = "# @exit-codes: 0 100\n";
        let annotation = parse(content).unwrap();
        assert_eq!(annotation.exit_codes, vec![0, 100]);
    }
}
